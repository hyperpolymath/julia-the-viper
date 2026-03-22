// SPDX-License-Identifier: PMPL-1.0-or-later
import * as vscode from 'vscode';
import * as path from 'path';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('Julia the Viper extension activated');

    // LSP client setup
    const config = vscode.workspace.getConfiguration('jtv');
    const lspPath = config.get<string>('lsp.path', 'jtv-lsp');

    const serverOptions: ServerOptions = {
        run: { command: lspPath },
        debug: { command: lspPath }
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'jtv' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.jtv')
        }
    };

    client = new LanguageClient(
        'jtv',
        'Julia the Viper Language Server',
        serverOptions,
        clientOptions
    );

    client.start();

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('jtv.run', runFile),
        vscode.commands.registerCommand('jtv.debug', debugFile),
        vscode.commands.registerCommand('jtv.format', formatFile)
    );
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function runFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const filePath = editor.document.uri.fsPath;
    const terminal = vscode.window.createTerminal('Julia the Viper');
    terminal.show();
    terminal.sendText(`jtv-cli run "${filePath}"`);
}

async function debugFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const filePath = editor.document.uri.fsPath;
    const terminal = vscode.window.createTerminal('JtV Debug');
    terminal.show();
    terminal.sendText(`jtv-debug "${filePath}"`);
}

async function formatFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const terminal = vscode.window.createTerminal('JtV Format');
    terminal.show();
    terminal.sendText(`jtv-cli format "${editor.document.uri.fsPath}"`);
}
