// SPDX-License-Identifier: MIT OR GPL-3.0-or-later OR Palimpsest-0.8
// SPDX-FileCopyrightText: 2025 Julia the Viper Contributors
//
// Julia the Viper - WASM Code Generator
// Compiles JtV bytecode IR to WebAssembly binary format

use crate::bytecode::{Opcode, CompiledModule, CompiledFunction, Value as BcValue};
use crate::error::{JtvError, Result};
use wasm_encoder::{
    Module, TypeSection, FunctionSection, CodeSection, ExportSection,
    MemorySection, MemoryType, Function, Instruction, ValType, ExportKind,
    GlobalSection, GlobalType, ConstExpr,
};

/// WASM code generator that compiles JtV bytecode to WebAssembly
pub struct WasmGenerator {
    module: Module,
    type_section: TypeSection,
    function_section: FunctionSection,
    export_section: ExportSection,
    code_section: CodeSection,
    memory_section: MemorySection,
    global_section: GlobalSection,
    function_count: u32,
    type_count: u32,
}

impl WasmGenerator {
    pub fn new() -> Self {
        WasmGenerator {
            module: Module::new(),
            type_section: TypeSection::new(),
            function_section: FunctionSection::new(),
            export_section: ExportSection::new(),
            code_section: CodeSection::new(),
            memory_section: MemorySection::new(),
            global_section: GlobalSection::new(),
            function_count: 0,
            type_count: 0,
        }
    }

    /// Compile a JtV bytecode module to WASM binary
    pub fn compile(&mut self, compiled_module: &CompiledModule) -> Result<Vec<u8>> {
        // Add memory (1 page = 64KB for stack and heap)
        self.memory_section.memory(MemoryType {
            minimum: 1,
            maximum: Some(16),
            memory64: false,
            shared: false,
        });

        // Add globals for stack pointer
        self.global_section.global(
            GlobalType {
                val_type: ValType::I32,
                mutable: true,
            },
            &ConstExpr::i32_const(0), // Stack pointer starts at 0
        );

        // Compile main function from top-level code if present
        if !compiled_module.code.is_empty() {
            self.compile_main_function(&compiled_module.code)?;
        }

        // Compile all user-defined functions
        for func in &compiled_module.functions {
            self.compile_function(func)?;
        }

        // Assemble the module
        let mut module = Module::new();
        module.section(&self.type_section);
        module.section(&self.function_section);
        module.section(&self.memory_section);
        module.section(&self.global_section);
        module.section(&self.export_section);
        module.section(&self.code_section);

        Ok(module.finish())
    }

    fn compile_main_function(&mut self, opcodes: &[Opcode]) -> Result<()> {
        // Add function type: () -> i64 (returns last result)
        self.type_section.function(vec![], vec![ValType::I64]);
        let type_idx = self.type_count;
        self.type_count += 1;

        // Add function
        self.function_section.function(type_idx);

        // Export as "_start"
        self.export_section.export("_start", ExportKind::Func, self.function_count);

        // Compile function body
        let mut func = Function::new(vec![(1, ValType::I64)]); // 1 local for result
        self.compile_opcodes(&mut func, opcodes)?;
        func.instruction(&Instruction::LocalGet(0)); // Return result
        func.instruction(&Instruction::End);

        self.code_section.function(&func);
        self.function_count += 1;

        Ok(())
    }

    fn compile_function(&mut self, func: &CompiledFunction) -> Result<()> {
        // Determine function signature based on arity
        let params: Vec<ValType> = (0..func.arity).map(|_| ValType::I64).collect();
        let results = vec![ValType::I64];

        self.type_section.function(params, results);
        let type_idx = self.type_count;
        self.type_count += 1;

        self.function_section.function(type_idx);

        // Export function by name
        self.export_section.export(&func.name, ExportKind::Func, self.function_count);

        // Compile function body
        // Local count = arity + local variables needed
        let local_count = func.locals.max(func.arity) as u32;
        let mut wasm_func = Function::new(vec![(local_count, ValType::I64)]);

        self.compile_opcodes(&mut wasm_func, &func.code)?;
        wasm_func.instruction(&Instruction::End);

        self.code_section.function(&wasm_func);
        self.function_count += 1;

        Ok(())
    }

    fn compile_opcodes(&mut self, func: &mut Function, opcodes: &[Opcode]) -> Result<()> {
        let mut pc = 0;

        while pc < opcodes.len() {
            match &opcodes[pc] {
                Opcode::Push(value) => {
                    match value {
                        BcValue::Int(n) => {
                            func.instruction(&Instruction::I64Const(*n));
                        }
                        BcValue::Bool(b) => {
                            func.instruction(&Instruction::I64Const(if *b { 1 } else { 0 }));
                        }
                        BcValue::Float(f) => {
                            // Store float as reinterpreted i64 bits
                            func.instruction(&Instruction::I64Const(f.to_bits() as i64));
                        }
                        _ => {
                            // For complex types, push a placeholder
                            func.instruction(&Instruction::I64Const(0));
                        }
                    }
                }

                Opcode::Pop => {
                    func.instruction(&Instruction::Drop);
                }

                Opcode::Dup => {
                    // WASM doesn't have native dup, use local
                    func.instruction(&Instruction::LocalTee(0));
                    func.instruction(&Instruction::LocalGet(0));
                }

                Opcode::LoadLocal(idx) => {
                    func.instruction(&Instruction::LocalGet(*idx));
                }

                Opcode::StoreLocal(idx) => {
                    func.instruction(&Instruction::LocalSet(*idx));
                }

                Opcode::LoadGlobal(idx) => {
                    func.instruction(&Instruction::GlobalGet(*idx));
                }

                Opcode::StoreGlobal(idx) => {
                    func.instruction(&Instruction::GlobalSet(*idx));
                }

                Opcode::Add => {
                    func.instruction(&Instruction::I64Add);
                }

                Opcode::Neg => {
                    // Negate: push the value, push 0, subtract (0 - value)
                    // But we need to reorder since we have value on stack
                    // Use: local.tee 0, i64.const 0, local.get 0, i64.sub
                    func.instruction(&Instruction::LocalTee(0));
                    func.instruction(&Instruction::Drop);
                    func.instruction(&Instruction::I64Const(0));
                    func.instruction(&Instruction::LocalGet(0));
                    func.instruction(&Instruction::I64Sub);
                }

                Opcode::Eq => {
                    func.instruction(&Instruction::I64Eq);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Ne => {
                    func.instruction(&Instruction::I64Ne);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Lt => {
                    func.instruction(&Instruction::I64LtS);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Le => {
                    func.instruction(&Instruction::I64LeS);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Gt => {
                    func.instruction(&Instruction::I64GtS);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Ge => {
                    func.instruction(&Instruction::I64GeS);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::And => {
                    func.instruction(&Instruction::I64And);
                }

                Opcode::Or => {
                    func.instruction(&Instruction::I64Or);
                }

                Opcode::Not => {
                    func.instruction(&Instruction::I64Eqz);
                    func.instruction(&Instruction::I64ExtendI32U);
                }

                Opcode::Jump(target) => {
                    // Calculate relative jump depth for WASM block structure
                    // WASM uses structured control flow, need to emit proper blocks
                    let depth = self.calculate_jump_depth(pc, *target as usize, opcodes);
                    func.instruction(&Instruction::Br(depth));
                }

                Opcode::JumpIfFalse(target) => {
                    // Conditional branch
                    func.instruction(&Instruction::I64Eqz);
                    func.instruction(&Instruction::I32WrapI64);
                    let depth = self.calculate_jump_depth(pc, *target as usize, opcodes);
                    func.instruction(&Instruction::BrIf(depth));
                }

                Opcode::JumpIfTrue(target) => {
                    func.instruction(&Instruction::I32WrapI64);
                    let depth = self.calculate_jump_depth(pc, *target as usize, opcodes);
                    func.instruction(&Instruction::BrIf(depth));
                }

                Opcode::Call(func_idx) => {
                    // Offset by 1 because main function is at index 0
                    func.instruction(&Instruction::Call(*func_idx + 1));
                }

                Opcode::Return => {
                    func.instruction(&Instruction::Return);
                }

                Opcode::Print => {
                    // Print would need to call an imported host function
                    // For now, just drop the value (no-op in pure WASM)
                    func.instruction(&Instruction::Drop);
                }

                Opcode::MakeList(count) => {
                    // Lists require memory allocation - for now, just handle count
                    // Drop all elements and push 0 (placeholder)
                    for _ in 0..*count {
                        func.instruction(&Instruction::Drop);
                    }
                    func.instruction(&Instruction::I64Const(0));
                }

                Opcode::MakeTuple(count) => {
                    // Similar to list
                    for _ in 0..*count {
                        func.instruction(&Instruction::Drop);
                    }
                    func.instruction(&Instruction::I64Const(0));
                }

                Opcode::BeginReverse | Opcode::EndReverse => {
                    // Reverse blocks are handled at compile time
                }

                Opcode::Halt => {
                    func.instruction(&Instruction::Unreachable);
                }
            }

            pc += 1;
        }

        Ok(())
    }

    fn calculate_jump_depth(&self, _from: usize, _to: usize, _opcodes: &[Opcode]) -> u32 {
        // Simplified: return 0 for now, proper implementation needs control flow analysis
        // WASM structured control flow requires converting to blocks/loops
        0
    }
}

impl Default for WasmGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level compilation function: source code -> WASM binary
pub fn compile_to_wasm(source: &str) -> Result<Vec<u8>> {
    use crate::parser::parse_program;
    use crate::bytecode::BytecodeCompiler;

    // Parse source
    let program = parse_program(source)?;

    // Compile to bytecode
    let mut bc_compiler = BytecodeCompiler::new();
    let compiled_module = bc_compiler.compile(&program)?;

    // Generate WASM
    let mut wasm_gen = WasmGenerator::new();
    wasm_gen.compile(&compiled_module)
}

/// Compile and write to file
pub fn compile_to_wasm_file(source: &str, output_path: &str) -> Result<()> {
    let wasm_bytes = compile_to_wasm(source)?;
    std::fs::write(output_path, wasm_bytes)
        .map_err(|e| JtvError::RuntimeError(format!("Failed to write WASM: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let source = "x = 5 + 3";
        let result = compile_to_wasm(source);
        assert!(result.is_ok());
        let wasm = result.unwrap();
        // WASM magic number: \0asm
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6D]);
    }

    #[test]
    fn test_compile_function() {
        let source = r#"
fn add(a: Int, b: Int): Int {
    return a + b
}
"#;
        let result = compile_to_wasm(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wasm_has_valid_header() {
        let source = "x = 1 + 2";
        let wasm = compile_to_wasm(source).unwrap();

        // Check WASM magic number
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        // Check WASM version (1)
        assert_eq!(&wasm[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }
}
