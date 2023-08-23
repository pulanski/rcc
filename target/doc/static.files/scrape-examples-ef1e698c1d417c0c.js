"use strict";
(function () {
    const DEFAULT_MAX_LINES = 5;
    const HIDDEN_MAX_LINES = 10;
    function scrollToLoc(elt, loc, isHidden) {
        const lines = elt.querySelector(".src-line-numbers");
        let scrollOffset;
        const maxLines = isHidden ? HIDDEN_MAX_LINES : DEFAULT_MAX_LINES;
        if (loc[1] - loc[0] > maxLines) {
            const line = Math.max(0, loc[0] - 1);
            scrollOffset = lines.children[line].offsetTop;
        } else {
            const wrapper = elt.querySelector(".code-wrapper");
            const halfHeight = wrapper.offsetHeight / 2;
            const offsetTop = lines.children[loc[0]].offsetTop;
            const lastLine = lines.children[loc[1]];
            const offsetBot = lastLine.offsetTop + lastLine.offsetHeight;
            const offsetMid = (offsetTop + offsetBot) / 2;
            scrollOffset = offsetMid - halfHeight;
        }
        lines.scrollTo(0, scrollOffset);
        elt.querySelector(".rust").scrollTo(0, scrollOffset);
    }
    function updateScrapedExample(example, isHidden) {
        const locs = JSON.parse(
            example.attributes.getNamedItem("data-locs").textContent,
        );
        let locIndex = 0;
        const highlights = Array.prototype.slice.call(
            example.querySelectorAll(".highlight"),
        );
        const link = example.querySelector(".scraped-example-title a");
        if (locs.length > 1) {
            const onChangeLoc = (changeIndex) => {
                removeClass(highlights[locIndex], "focus");
                changeIndex();
                scrollToLoc(example, locs[locIndex][0], isHidden);
                addClass(highlights[locIndex], "focus");
                const url = locs[locIndex][1];
                const title = locs[locIndex][2];
                link.href = url;
                link.innerHTML = title;
            };
            example.querySelector(".prev").addEventListener("click", () => {
                onChangeLoc(() => {
                    locIndex = (locIndex - 1 + locs.length) % locs.length;
                });
            });
            example.querySelector(".next").addEventListener("click", () => {
                onChangeLoc(() => {
                    locIndex = (locIndex + 1) % locs.length;
                });
            });
        }
        const expandButton = example.querySelector(".expand");
        if (expandButton) {
            expandButton.addEventListener("click", () => {
                if (hasClass(example, "expanded")) {
                    removeClass(example, "expanded");
                    scrollToLoc(example, locs[0][0], isHidden);
                } else {
                    addClass(example, "expanded");
                }
            });
        }
        scrollToLoc(example, locs[0][0], isHidden);
    }
    const firstExamples = document.querySelectorAll(
        ".scraped-example-list > .scraped-example",
    );
    onEachLazy(firstExamples, (el) => updateScrapedExample(el, false));
    onEachLazy(document.querySelectorAll(".more-examples-toggle"), (toggle) => {
        onEachLazy(
            toggle.querySelectorAll(".toggle-line, .hide-more"),
            (button) => {
                button.addEventListener("click", () => {
                    toggle.open = false;
                });
            },
        );
        const moreExamples = toggle.querySelectorAll(".scraped-example");
        toggle.querySelector("summary").addEventListener(
            "click",
            () => {
                setTimeout(() => {
                    onEachLazy(moreExamples, (el) =>
                        updateScrapedExample(el, true),
                    );
                });
            },
            { once: true },
        );
    });
})();
