import * as vscode from "vscode";
import { foldingRange } from "./ffi";

export class ViewFoldingRangeProvider implements vscode.FoldingRangeProvider {
    provideFoldingRanges(
        document: vscode.TextDocument,
        context: vscode.FoldingContext,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.FoldingRange[]> {
        const foldingRanges: vscode.FoldingRange[] = [];

        let ranges = foldingRange(document.uri.fsPath);
        for (let i = 0; i < ranges.length; i += 2) {
            const start = ranges[i];
            const end = ranges[i + 1];
            foldingRanges.push(
                new vscode.FoldingRange(
                    start,
                    end,
                    vscode.FoldingRangeKind.Region
                )
            );
        }

        return foldingRanges;
    }
}
