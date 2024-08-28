import * as vscode from "vscode";

export class ViewFoldingRangeProvider implements vscode.FoldingRangeProvider {
    provideFoldingRanges(
        document: vscode.TextDocument,
        context: vscode.FoldingContext,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.FoldingRange[]> {
        // document.uri.fsPath;
        const foldingRanges: vscode.FoldingRange[] = [];
        const text = document.getText();
        const regex = /<div>([\s\S]*?)<\/div>/g;
        // const viewMacroStartRegex = /view!\s*\{/g;

        let match;
        while ((match = regex.exec(text)) !== null) {
            const start = document.positionAt(match.index);
            const end = document.positionAt(match.index + match[0].length);
            foldingRanges.push(
                new vscode.FoldingRange(
                    start.line,
                    end.line,
                    vscode.FoldingRangeKind.Region
                )
            );
        }

        return foldingRanges;
    }
}
