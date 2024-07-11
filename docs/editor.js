;(function () {
  var old = console.log
  var logger = document.getElementById("log")
  console.log = function (message) {
    if (typeof message == "object") {
      logger.innerHTML +=
        (JSON && JSON.stringify ? JSON.stringify(message) : message) + "<br />"
    } else {
      logger.innerHTML += message + "<br />"
    }
  }
})()

require.config({
  paths: { vs: "https://unpkg.com/monaco-editor@latest/min/vs" },
})
window.MonacoEnvironment = { getWorkerUrl: () => proxy }

let proxy = URL.createObjectURL(
  new Blob(
    [
      `
	self.MonacoEnvironment = {
		baseUrl: 'https://unpkg.com/monaco-editor@latest/min/'
	};
	importScripts('https://unpkg.com/monaco-editor@latest/min/vs/base/worker/workerMain.js');
`,
    ],
    { type: "text/javascript" }
  )
)

const defultValue = `const a = 9

if (a > 10) {
  print("a is greater than 10")
} else {
  print("a is less than 10")
}`

require(["vs/editor/editor.main"], function () {
  window.editor = monaco.editor.create(document.getElementById("container"), {
    value: defultValue,
    language: "javascript",
    fontSize: "18px",
    theme: "vs-dark",
    automaticLayout: true,
  })
})

function runCode() {
  document.getElementById("log").innerHTML = ""
  var value = window.editor.getValue()
  run(value)
}
