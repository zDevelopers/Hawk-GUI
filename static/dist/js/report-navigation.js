document.addEventListener("DOMContentLoaded", function() {
    "use strict";

    const tabs = document.querySelectorAll('.js-tab-section')
    const tabs_links = document.querySelectorAll('.js-tab-link')
    const players_tabs_links = document.querySelectorAll('nav.players-statistics-navigation ul li')

    /**
     * Switches to another section.
     *
     * @param {string} target A section ID, optionally followed by “:” then a specifier.
     *                       The specifier is only supported for `players:` (the player name,
     *                       e.g. `players:Jenjeur` to switch to Jenjeur's player section).
     * @param {boolean|undefined} no_history If true, history won't be altered. Used when restoring state
     *                                       on backward user navigation.
     */
    function switch_to(target, no_history) {
        let tab;
        let sub;

        if (target.indexOf(":") !== false) {
            target = target.split(":");
            tab = target[0];
            sub = target.slice(1).join(":");
        }
        else {
            tab = target;
            sub = undefined;
        }

        tabs.forEach(function (tab) {
            tab.classList.add('is-hidden');
        });

        let tab_el = document.getElementById(tab);

        if (tab_el) {
            tab_el.classList.remove('is-hidden');

            tabs_links.forEach(function (tab_link) {
                tab_link.parentElement.classList.remove('is-active');
                if (tab_link.getAttribute('href') === "#" + tab) {
                    tab_link.parentElement.classList.add('is-active');
                }
            })
        }

        if (sub && tab === 'players') {
            switch_to_player(sub, no_history)
        } else if (!no_history) {
            if (tab === 'summary') {
                history.pushState({}, '', '#')
            } else {
                history.pushState({}, '', "#" + tab)
            }
        }
    }

    /**
     * Switch to a player tab.
     * @param {string} player The player name, or a falsy value to go to the global statistics.
     * @param {boolean|undefined} no_history If true, history won't be altered. Used when restoring state
     *                                       on backward user navigation.
     */
    function switch_to_player(player, no_history) {
        if (!player) {
            player = '~global'
        }

        players_tabs_links.forEach(function(tab_link) {
            const anchor = tab_link.querySelector("a")
            const tab_container_anchor = anchor.getAttribute("href")
            const tab_container = document.getElementById(tab_container_anchor.replace('#', ''))

            const target_anchor = `#players:${player}`

            tab_link.classList.toggle("is-active", tab_container_anchor === target_anchor)
            tab_container.classList.toggle("is-active", tab_container_anchor === target_anchor)
        })

        if (!no_history) {
            if (player === '~global') {
                history.pushState({}, '', '#players')
            } else {
                history.pushState({}, '', `#players:${player}`)
            }
        }
    }

    function switch_from_hash() {
        const hash = document.location.hash.replace('#', '')
        switch_to(hash ? hash : 'summary', true)
    }

    tabs_links.forEach(function (element) {
        element.addEventListener("click", function (e) {
            e.preventDefault()
            switch_to(e.target.getAttribute('href').replace("#", ""))
        });
    });

    players_tabs_links.forEach(function(element) {
        const anchor = element.querySelector("a")
        const tab_container_anchor = anchor.getAttribute("href")
        const tab_container = document.getElementById(tab_container_anchor.replace('#', ''))

        element.classList.toggle("is-active", tab_container_anchor === "#players:~global")
        tab_container.classList.toggle("is-active", tab_container_anchor === "#players:~global")

        anchor.addEventListener('click', function (e) {
            e.preventDefault()
            if (tab_container_anchor === "#players:~global") {
                switch_to_player()
            } else {
                switch_to_player(tab_container_anchor.replace('#players:', ''))
            }
        })
    })

    const players_links = document.querySelectorAll('a.player');

    players_links.forEach(function (element) {
        element.addEventListener('click', function (e) {
            e.preventDefault();
            element.blur();

            const player = element.getAttribute("data-player")
            if (player) {
                switch_to(`players:${player}`)
            }
        })
    })

    switch_from_hash()

    window.onpopstate = switch_from_hash
    window.onhashchange = switch_from_hash
})
