const { invoke } = window.__TAURI__.tauri;

const $ = document.querySelector.bind(document)
const $$ = document.querySelectorAll.bind(document)

window.addEventListener("DOMContentLoaded", async function ()
{
	$("#test").addEventListener("click", (e) =>
		invoke("rename", {
			search: $("#search-for").value,
			replace: $("#replace-with").value,
			useRegex: $("#use-regex").checked,
			matchAll: $("#match-all").checked,
			caseSensitive: $("#case-sensitive").checked
		}));
});
