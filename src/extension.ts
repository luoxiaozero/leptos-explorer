import * as vscode from "vscode";
import { ViewFoldingRangeProvider } from "./foldingProvider";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
    // The command has been defined in the package.json file
    // Now provide the implementation of the command with registerCommand
    // The commandId parameter must match the command field in package.json
    const disposable = vscode.commands.registerCommand(
        "leptos-explorer.helloWorld",
        () => {
            // The code you place here will be executed every time your command is executed
            // Display a message box to the user
            vscode.window.showInformationMessage(
                "Hello World from Leptos Explore2!"
            );
        }
    );

    context.subscriptions.push(disposable);
    context.subscriptions.push(
        vscode.languages.registerFoldingRangeProvider(
            "rust",
            new ViewFoldingRangeProvider()
        )
    );
}

// This method is called when your extension is deactivated
export function deactivate() {}
