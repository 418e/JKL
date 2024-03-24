import * as path from "path";
import {
  workspace,
  ExtensionContext,
  languages,
  DocumentFormattingEditProvider,
  TextDocument,
  FormattingOptions,
  CancellationToken,
  ProviderResult,
  TextEdit,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;
export function activate(context: ExtensionContext) {
  const serverModule = context.asAbsolutePath(
    path.join("server", "out", "server.js")
  );
  const serverOptions: ServerOptions = {
    run: { module: serverModule, transport: TransportKind.ipc },
    debug: {
      module: serverModule,
      transport: TransportKind.ipc,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "tron" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
    },
  };

  client = new LanguageClient(
    "languageServerExample",
    "Language Server Example",
    serverOptions,
    clientOptions
  );
  client.start().then(() => {
    context.subscriptions.push(
      languages.registerDocumentFormattingEditProvider("tron", {
        provideDocumentFormattingEdits(
          document: TextDocument,
          options: FormattingOptions,
          token: CancellationToken
        ): ProviderResult<TextEdit[]> {
          return client.sendRequest(
            "textDocument/formatting",
            {
              textDocument: { uri: document.uri.toString() },
              options: options,
            },
            token
          );
        },
      })
    );
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
