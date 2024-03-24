import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  InitializeParams,
  DidChangeConfigurationNotification,
  CompletionItem,
  CompletionItemKind,
  TextDocumentPositionParams,
  TextDocumentSyncKind,
  InitializeResult,
  TextEdit,
  FormattingOptions,
  Range,
} from "vscode-languageserver/node";

import { TextDocument } from "vscode-languageserver-textdocument";

const connection = createConnection(ProposedFeatures.all);
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;

connection.onInitialize((params: InitializeParams) => {
  const capabilities = params.capabilities;
  hasConfigurationCapability = !!(
    capabilities.workspace && !!capabilities.workspace.configuration
  );
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );

  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
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
    connection.client.register(
      DidChangeConfigurationNotification.type,
      undefined
    );
  }
});


interface VariableDeclaration {
  declaration: string;
  type: string;
  value: string;
}

const variableDeclarations = new Map<string, VariableDeclaration>();

function trackVariableDeclarations(document: TextDocument): void {
  const text = document.getText();
  const regex =
    /let\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*([^;]*);/g;
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

interface Parameter {
  name: string;
  type: string;
}

function extractParameters(functionDeclaration: string): Parameter[] {
  const regex = /fn\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(([^)]*)\)/;
  const match = functionDeclaration.match(regex);
  if (!match) {
    return [];
  }

  const paramsString = match[1];
  const paramsRegex =
    /\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*/g;
  let paramsMatch;
  const parameters: Parameter[] = [];

  while ((paramsMatch = paramsRegex.exec(paramsString)) !== null) {
    parameters.push({
      name: paramsMatch[1],
      type: paramsMatch[2],
    });
  }

  return parameters;
}

interface FunctionDeclaration {
  declaration: string;
  parameters: Parameter[];
  returnType: string;
}

const functionDeclarations = new Map<string, FunctionDeclaration>();

function trackFunctionDeclarations(document: TextDocument): void {
  const text = document.getText();
  const regex =
    /fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\{[^]*?\}/g;
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

connection.onCompletion(
  (_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {

    const completions: CompletionItem[] = [
      {
        label: "break",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "else if",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "else",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "false",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "for",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "fn",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "if",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "use",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "null",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "true",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "let",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "while",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "case",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "default",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "switch",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "number",
        kind: CompletionItemKind.Keyword,
      },
      {
        label: "string",
        kind: CompletionItemKind.Keyword,
      },
    ];
    variableDeclarations.forEach((declaration, variableName) => {
      completions.push({
        label: variableName,
        kind: CompletionItemKind.Variable,
        detail: declaration.declaration,
      });
    });
    functionDeclarations.forEach((declaration, functionName) => {
      completions.push({
        label: functionName,
        kind: CompletionItemKind.Function,
        detail: declaration.declaration,
      });
      declaration.parameters.forEach((param) => {
        completions.push({
          label: param.name,
          kind: CompletionItemKind.Variable
        })
      } )
    });
    return completions;
  }
);
connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
  return item;
});

function formatTronDocument(text: string, options: FormattingOptions): string {
  const lines = text.split(/\r?\n/);
  const indentStack: string[] = [];
  function getIndentation(line: string): string {
    return indentStack[indentStack.length - 1] || "";
  }

  function formatLine(line: string): string {
    const trimmedLine = line.trim();
    if (trimmedLine === "") {
      return "";
    } else {
      let indentation = getIndentation(trimmedLine);
      if (trimmedLine.endsWith("{")) {
        indentStack.push(indentation + "    ");
      } else if (trimmedLine === "}") {
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

connection.onDocumentFormatting((params): TextEdit[] => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return [];
  }

  const text = document.getText();
  const formattedText = formatTronDocument(text, params.options);

  return [
    TextEdit.replace(
      Range.create(document.positionAt(0), document.positionAt(text.length)),
      formattedText
    ),
  ];
});

documents.listen(connection);
connection.listen();
