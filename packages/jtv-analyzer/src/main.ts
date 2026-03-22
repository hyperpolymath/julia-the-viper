// Julia the Viper - Code Analyzer for Python/JavaScript/Ruby
// Analyzes legacy code for termination guarantees, purity, and complexity

export interface AnalysisResult {
  file: string;
  language: "python" | "javascript" | "ruby";
  functions: FunctionAnalysis[];
  warnings: Warning[];
  suggestions: Suggestion[];
}

export interface FunctionAnalysis {
  name: string;
  line: number;
  isPure: boolean;
  terminates: boolean;
  complexity: ComplexityAnalysis;
  dataOperations: DataOperation[];
  controlFlow: ControlFlowNode[];
}

export interface ComplexityAnalysis {
  cyclomaticComplexity: number;
  cognitiveComplexity: number;
  loops: number;
  conditionals: number;
  recursion: boolean;
}

export interface DataOperation {
  type: "add" | "subtract" | "multiply" | "divide" | "other";
  line: number;
  safe: boolean;  // Can be converted to JtV addition-only?
}

export interface ControlFlowNode {
  type: "if" | "while" | "for" | "function_call" | "return";
  line: number;
  terminating: boolean;
}

export interface Warning {
  severity: "error" | "warning" | "info";
  message: string;
  line: number;
  suggestion?: string;
}

export interface Suggestion {
  type: "extract_to_jtv" | "mark_pure" | "bound_loop";
  message: string;
  line: number;
  codeSnippet: string;
  jtvEquivalent?: string;
}

export class JtvAnalyzer {
  private language: "python" | "javascript" | "ruby";

  constructor(language: "python" | "javascript" | "ruby") {
    this.language = language;
  }

  async analyzeFile(filePath: string): Promise<AnalysisResult> {
    const code = await Deno.readTextFile(filePath);
    return this.analyzeCode(code, filePath);
  }

  analyzeCode(code: string, filePath: string = "unknown"): AnalysisResult {
    const result: AnalysisResult = {
      file: filePath,
      language: this.language,
      functions: [],
      warnings: [],
      suggestions: [],
    };

    const lines = code.split("\n");

    // Simple pattern-based analysis (real implementation would use AST)
    let inFunction = false;
    let currentFunction: Partial<FunctionAnalysis> | null = null;
    let braceDepth = 0;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      const lineNum = i + 1;

      // Detect function declarations
      if (this.isFunctionDeclaration(line)) {
        if (currentFunction) {
          result.functions.push(currentFunction as FunctionAnalysis);
        }

        const funcName = this.extractFunctionName(line);
        currentFunction = {
          name: funcName,
          line: lineNum,
          isPure: true,  // Assume pure until proven otherwise
          terminates: true,  // Assume terminates until proven otherwise
          complexity: {
            cyclomaticComplexity: 1,
            cognitiveComplexity: 0,
            loops: 0,
            conditionals: 0,
            recursion: false,
          },
          dataOperations: [],
          controlFlow: [],
        };
        inFunction = true;
      }

      if (!inFunction || !currentFunction) continue;

      // Analyze data operations
      if (line.includes("+") && !line.includes("++")) {
        currentFunction.dataOperations!.push({
          type: "add",
          line: lineNum,
          safe: true,  // Addition is safe for JtV
        });
      } else if (line.includes("*") || line.includes("/")) {
        currentFunction.dataOperations!.push({
          type: line.includes("*") ? "multiply" : "divide",
          line: lineNum,
          safe: false,  // Not directly supported in Data Language
        });

        result.suggestions.push({
          type: "extract_to_jtv",
          message: `Consider extracting pure computation to JtV for guaranteed termination`,
          line: lineNum,
          codeSnippet: line,
          jtvEquivalent: this.convertToJtv(line),
        });
      }

      // Detect control flow
      if (line.includes("if") || line.includes("else")) {
        currentFunction.complexity!.conditionals++;
        currentFunction.complexity!.cyclomaticComplexity++;
        currentFunction.complexity!.cognitiveComplexity++;
        currentFunction.controlFlow!.push({
          type: "if",
          line: lineNum,
          terminating: true,
        });
      }

      if (line.includes("while") || line.includes("for")) {
        currentFunction.complexity!.loops++;
        currentFunction.complexity!.cyclomaticComplexity++;
        currentFunction.complexity!.cognitiveComplexity += 2;
        currentFunction.terminates = false;  // Unbounded loop
        currentFunction.controlFlow!.push({
          type: line.includes("while") ? "while" : "for",
          line: lineNum,
          terminating: false,
        });

        result.warnings.push({
          severity: "warning",
          message: "Unbounded loop detected - termination not guaranteed",
          line: lineNum,
          suggestion: "Add loop bound or extract to JtV with range-based iteration",
        });
      }

      // Detect side effects
      if (
        line.includes("print") ||
        line.includes("console.log") ||
        line.includes("puts") ||
        line.includes("write") ||
        line.includes("fetch") ||
        line.includes("axios")
      ) {
        currentFunction.isPure = false;

        result.warnings.push({
          severity: "info",
          message: "Function has side effects (I/O operations)",
          line: lineNum,
          suggestion: "Consider separating pure computation from side effects",
        });
      }

      // Detect recursion
      if (currentFunction.name && line.includes(currentFunction.name + "(")) {
        currentFunction.complexity!.recursion = true;
        currentFunction.terminates = false;

        result.warnings.push({
          severity: "warning",
          message: "Recursive function - termination not guaranteed",
          line: lineNum,
          suggestion: "Convert to iterative form or prove termination",
        });
      }

      // Track brace depth for function boundaries
      braceDepth += (line.match(/{/g) || []).length;
      braceDepth -= (line.match(/}/g) || []).length;

      if (braceDepth === 0 && currentFunction) {
        result.functions.push(currentFunction as FunctionAnalysis);
        currentFunction = null;
        inFunction = false;
      }
    }

    // Add final function if still in progress
    if (currentFunction) {
      result.functions.push(currentFunction as FunctionAnalysis);
    }

    // Generate overall suggestions
    this.generateSuggestions(result);

    return result;
  }

  private isFunctionDeclaration(line: string): boolean {
    switch (this.language) {
      case "python":
        return line.startsWith("def ");
      case "javascript":
        return line.includes("function ") || line.includes("=> ");
      case "ruby":
        return line.startsWith("def ");
      default:
        return false;
    }
  }

  private extractFunctionName(line: string): string {
    const match = line.match(/(?:def|function)\s+(\w+)/);
    return match ? match[1] : "anonymous";
  }

  private convertToJtv(line: string): string {
    // Simplified conversion - real implementation would be more sophisticated
    if (line.includes("*")) {
      return "// Use loop with addition in JtV: for i in 0..n { sum = sum + x }";
    }
    if (line.includes("/")) {
      return "// Division not directly supported - use rational type or iterative subtraction";
    }
    return line.replace(/\*/g, "+").replace(/\//g, "+");
  }

  private generateSuggestions(result: AnalysisResult): void {
    const pureFunctions = result.functions.filter((f) => f.isPure && f.terminates);

    if (pureFunctions.length > 0) {
      result.suggestions.push({
        type: "extract_to_jtv",
        message: `${pureFunctions.length} pure, terminating functions found - excellent candidates for JtV extraction`,
        line: 0,
        codeSnippet: pureFunctions.map((f) => f.name).join(", "),
      });
    }

    const highComplexity = result.functions.filter(
      (f) => f.complexity.cyclomaticComplexity > 10
    );
    if (highComplexity.length > 0) {
      result.warnings.push({
        severity: "warning",
        message: `${highComplexity.length} functions with high complexity - consider refactoring`,
        line: 0,
      });
    }
  }

  printReport(result: AnalysisResult): void {
    console.log(`\n=== JtV Analysis Report: ${result.file} ===\n`);

    console.log(`Language: ${result.language}`);
    console.log(`Functions analyzed: ${result.functions.length}\n`);

    // Summary
    const pure = result.functions.filter((f) => f.isPure).length;
    const terminating = result.functions.filter((f) => f.terminates).length;

    console.log(`Pure functions: ${pure}/${result.functions.length}`);
    console.log(`Terminating functions: ${terminating}/${result.functions.length}\n`);

    // Warnings
    if (result.warnings.length > 0) {
      console.log(`Warnings (${result.warnings.length}):`);
      for (const warning of result.warnings) {
        const icon = warning.severity === "error" ? "❌" : warning.severity === "warning" ? "⚠️" : "ℹ️";
        console.log(`  ${icon} Line ${warning.line}: ${warning.message}`);
        if (warning.suggestion) {
          console.log(`     → ${warning.suggestion}`);
        }
      }
      console.log();
    }

    // Suggestions
    if (result.suggestions.length > 0) {
      console.log(`Suggestions (${result.suggestions.length}):`);
      for (const suggestion of result.suggestions) {
        console.log(`  💡 Line ${suggestion.line}: ${suggestion.message}`);
        if (suggestion.jtvEquivalent) {
          console.log(`     JtV: ${suggestion.jtvEquivalent}`);
        }
      }
      console.log();
    }

    // Function details
    console.log("Function Details:");
    for (const func of result.functions) {
      const pureIcon = func.isPure ? "✓" : "✗";
      const termIcon = func.terminates ? "✓" : "✗";
      console.log(`\n  ${func.name} (line ${func.line})`);
      console.log(`    Pure: ${pureIcon} | Terminates: ${termIcon}`);
      console.log(`    Complexity: ${func.complexity.cyclomaticComplexity} (cyclomatic)`);
      console.log(`    Loops: ${func.complexity.loops} | Conditionals: ${func.complexity.conditionals}`);
    }

    console.log("\n=== End Report ===\n");
  }
}

// CLI entry point
if (import.meta.main) {
  const args = Deno.args;

  if (args.length === 0) {
    console.log("Usage: deno run --allow-read main.ts <file> [language]");
    console.log("Languages: python, javascript, ruby");
    Deno.exit(1);
  }

  const filePath = args[0];
  const language = (args[1] || "javascript") as "python" | "javascript" | "ruby";

  const analyzer = new JtvAnalyzer(language);
  const result = await analyzer.analyzeFile(filePath);
  analyzer.printReport(result);
}

export default JtvAnalyzer;
