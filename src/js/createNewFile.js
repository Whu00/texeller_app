//const invoke = window.__TAURI__.invoke
//
//invoke('my_custom_command').then(message => console.log(message)) //import * as std from 'std'
tauri
	.invoke('invoke_js_function', { message: 'Hello from JavaScript!' })
	.then(response => {
		console.log(response) // Handle the response from Rust
	})
	.catch(error => {
		console.error(error) // Handle any errors
	})

//async function createNewFile() {
//	returnconsole.log('All works')
//}

//const tabsContent = document.querySelectorAll('.newFileWindow')

// When using the Tauri API npm package:
