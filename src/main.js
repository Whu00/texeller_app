const { app, Menu } = require('@tauri-apps/api')

const menuTemplate = [
	{
		label: 'File',
		submenu: [
			{
				label: 'Open',
				accelerator: 'CmdOrCtrl+O',
				click: () => {
					// Добавьте обработчик для действия "Open"
				},
			},
			{
				label: 'Save',
				accelerator: 'CmdOrCtrl+S',
				click: () => {
					// Добавьте обработчик для действия "Save"
				},
			},
		],
	},
	{
		label: 'Edit',
		submenu: [
			{
				label: 'Cut',
				accelerator: 'CmdOrCtrl+X',
				click: () => {
					// Добавьте обработчик для действия "Cut"
				},
			},
			{
				label: 'Copy',
				accelerator: 'CmdOrCtrl+C',
				click: () => {
					// Добавьте обработчик для действия "Copy"
				},
			},
			{
				label: 'Paste',
				accelerator: 'CmdOrCtrl+V',
				click: () => {
					// Добавьте обработчик для действия "Paste"
				},
			},
		],
	},
]

app.on('tauri://create-menu', () => {
	const myMenu = Menu.buildFromTemplate(menuTemplate)
	Menu.setApplicationMenu(myMenu)
})
