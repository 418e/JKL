"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("vscode-languageserver/node");
const vscode_languageserver_textdocument_1 = require("vscode-languageserver-textdocument");
const connection = (0, node_1.createConnection)(node_1.ProposedFeatures.all);
const documents = new node_1.TextDocuments(vscode_languageserver_textdocument_1.TextDocument);
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;
connection.onInitialize((params) => {
    const capabilities = params.capabilities;
    hasConfigurationCapability = !!(capabilities.workspace && !!capabilities.workspace.configuration);
    hasWorkspaceFolderCapability = !!(capabilities.workspace && !!capabilities.workspace.workspaceFolders);
    const result = {
        capabilities: {
            textDocumentSync: node_1.TextDocumentSyncKind.Incremental,
            completionProvider: {
                resolveProvider: true,
            },
        },
    };
    if (hasWorkspaceFolderCapability) {
        result.capabilities.workspace = {
            workspaceFolders: {
                supported: true,
            },
        };
    }
    return result;
});
connection.onInitialized(() => {
    if (hasConfigurationCapability) {
        connection.client.register(node_1.DidChangeConfigurationNotification.type, undefined);
    }
});
const variableDeclarations = new Map();
function trackVariableDeclarations(document) {
    const text = document.getText();
    const regex = /let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([^;]*);/g;
    let match;
    while ((match = regex.exec(text)) !== null) {
        const variableName = match[1];
        const variableType = match[2];
        const variableValue = match[3];
        variableDeclarations.set(variableName, {
            declaration: match[0],
            type: variableType,
            value: variableValue,
        });
    }
}
function extractParameters(functionDeclaration) {
    const regex = /fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(([^)]*)\)/;
    const match = functionDeclaration.match(regex);
    if (!match) {
        return [];
    }
    const paramsString = match[1];
    const paramsRegex = /\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*/g;
    let paramsMatch;
    const parameters = [];
    while ((paramsMatch = paramsRegex.exec(paramsString)) !== null) {
        parameters.push({
            name: paramsMatch[1],
            type: paramsMatch[2],
        });
    }
    return parameters;
}
const functionDeclarations = new Map();
function trackFunctionDeclarations(document) {
    const text = document.getText();
    const regex = /fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\{[^]*?\}/g;
    let match;
    while ((match = regex.exec(text)) !== null) {
        const functionName = match[1];
        const functionDeclaration = match[0];
        const parameters = extractParameters(functionDeclaration);
        const returnType = match[3];
        functionDeclarations.set(functionName, {
            declaration: functionDeclaration,
            parameters,
            returnType,
        });
    }
}
documents.onDidChangeContent((change) => {
    trackVariableDeclarations(change.document);
    trackFunctionDeclarations(change.document);
});
connection.onCompletion((_textDocumentPosition) => {
    const completions = [
        {
            label: "break",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "else if",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "else",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "false",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "for",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "fn",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "if",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "use",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "null",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "true",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "let",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "while",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "case",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "default",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "switch",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "number",
            kind: node_1.CompletionItemKind.Keyword,
        },
        {
            label: "string",
            kind: node_1.CompletionItemKind.Keyword,
        },
    ];
    variableDeclarations.forEach((declaration, variableName) => {
        completions.push({
            label: variableName,
            kind: node_1.CompletionItemKind.Variable,
            detail: declaration.declaration,
        });
    });
    functionDeclarations.forEach((declaration, functionName) => {
        completions.push({
            label: functionName,
            kind: node_1.CompletionItemKind.Function,
            detail: declaration.declaration,
        });
        declaration.parameters.forEach((param) => {
            completions.push({
                label: param.name,
                kind: node_1.CompletionItemKind.Variable
            });
        });
    });
    return completions;
});
connection.onCompletionResolve((item) => {
    return item;
});
function formatTronDocument(text, options) {
    const lines = text.split(/\r?\n/);
    const indentStack = [];
    function getIndentation(line) {
        return indentStack[indentStack.length - 1] || "";
    }
    function formatLine(line) {
        const trimmedLine = line.trim();
        if (trimmedLine === "") {
            return "";
        }
        else {
            let indentation = getIndentation(trimmedLine);
            if (trimmedLine.endsWith("{")) {
                indentStack.push(indentation + "    ");
            }
            else if (trimmedLine === "}") {
                indentStack.pop();
                indentation = getIndentation(trimmedLine);
            }
            return indentation + trimmedLine;
        }
    }
    const formattedLines = lines.map(formatLine);
    const formattedText = formattedLines.join("\n");
    if (indentStack.length > 0) {
        throw new Error("Mismatched curly braces in the document");
    }
    return formattedText;
}
connection.onDocumentFormatting((params) => {
    const document = documents.get(params.textDocument.uri);
    if (!document) {
        return [];
    }
    const text = document.getText();
    const formattedText = formatTronDocument(text, params.options);
    return [
        node_1.TextEdit.replace(node_1.Range.create(document.positionAt(0), document.positionAt(text.length)), formattedText),
    ];
});
documents.listen(connection);
connection.listen();
//# sourceMappingURL=server.js.map