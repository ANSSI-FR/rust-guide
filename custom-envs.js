let gotochecklist = document.getElementsByClassName("gotochecklist")
var mySpan = document.createElement("span");
mySpan.innerHTML = "<a href=\"checklist.html\">" + checklist_ref + "</a>";
for (let index = 0; index < gotochecklist.length; index++) {
	const element = gotochecklist[index];
	element.parentNode.replaceChild(mySpan, element);
}