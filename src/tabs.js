const tabsTitle = document.querySelectorAll('.tabsButton')
const tabsContent = document.querySelectorAll('.tabContent')

tabsTitle.forEach(item =>
	item.addEventListener('click', event => {
		const tabsTitleTarget = event.target.getAttribute('data-tab')
		tabsTitle.forEach(element => element.classList.remove('activeTab'))
		tabsContent.forEach(element => element.classList.add('hiddenTabContent'))
		item.classList.add('activeTab')
		document
			.getElementById(tabsTitleTarget)
			.classList.remove('hiddenTabContent')
	})
)
