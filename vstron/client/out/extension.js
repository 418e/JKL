"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
var path = require("path");
var vscode_1 = require("vscode");
var node_1 = require("vscode-languageclient/node");
var client;
function activate(context) {
    var serverModule = context.asAbsolutePath(path.join("server", "out", "server.js"));
    var serverOptions = {
        run: { module: serverModule, transport: node_1.TransportKind.ipc },
        debug: {
            module: serverModule,
            transport: node_1.TransportKind.ipc,
        },
    };
    var clientOptions = {
        documentSelector: [{ scheme: "file", language: "tron" }],
        synchronize: {
            fileEvents: vscode_1.workspace.createFileSystemWatcher("**/.clientrc"),
        },
    };
    client = new node_1.LanguageClient("languageServerExample", "Language Server Example", serverOptions, clientOptions);
    client.start().then(function () {
        context.subscriptions.push(vscode_1.languages.registerDocumentFormattingEditProvider("tron", {
            provideDocumentFormattingEdits: function (document, options, token) {
                return client.sendRequest("textDocument/formatting", {
                    textDocument: { uri: document.uri.toString() },
                    options: options,
                }, token);
            },
        }));
    });
}
exports.activate = activate;
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
exports.deactivate = deactivate;
