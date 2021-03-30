document.addEventListener("DOMContentLoaded", function() {
    'use strict';

    // Replaces marked titles with aria-label, because they interfere with tooltips
    document.querySelectorAll("[data-replace-title]").forEach(function(element) {
        element.setAttribute("aria-label", element.getAttribute("title"));
        element.setAttribute("title", "");
    });

    const real_tooltip = document.createElement("div");
    real_tooltip.classList.add("minecraft-rich-tooltip");

    document.body.appendChild(real_tooltip);

    document.querySelectorAll("[aria-describedby]").forEach(function(element) {
        const tooltip = document.getElementById(element.getAttribute("aria-describedby"));

        element.addEventListener('mouseover', function(e) {
            tooltip.setAttribute("aria-hidden", "false");
            show_tooltip(real_tooltip, tooltip);
        });

        element.addEventListener('mouseout', function(e) {
            tooltip.setAttribute("aria-hidden", "true");
            hide_tooltip(real_tooltip, tooltip);
        });

        element.addEventListener('mousemove', function(e) {
            if (e.movementX && e.movementX > 20 || e.movementY && e.movementY > 20)
                return;

            let mouseX, mouseY;

            if (e.offsetX) {
                mouseX = e.offsetX;
                mouseY = e.offsetY;
            } else if (e.layerX) {
                mouseX = e.layerX;
                mouseY = e.layerY;
            }

            let body_bouncing_box    = document.body.getBoundingClientRect(),
                element_bouncing_box = element.getBoundingClientRect(),
                offset_top           = element_bouncing_box.top - body_bouncing_box.top,
                offset_left          = element_bouncing_box.left - body_bouncing_box.left;

            real_tooltip.style.top = (offset_top + mouseY - 4) + "px";
            real_tooltip.style.left = (offset_left + mouseX + 8) + "px";

        });
    });

    document.querySelectorAll(".is-hidden-statistics").forEach(function(element) {
        element.classList.add("is-hidden");
    });

    document.querySelectorAll(".toggle-hidden-statistics").forEach(function(element) {
        element.addEventListener("click", function(e) {
            e.preventDefault();

            element.classList.toggle("is-toggled");
            element.nextElementSibling.classList.toggle("is-hidden");

            element.blur();
        });
    });
});

function show_tooltip(real_tooltip, element) {
    'use strict';

    real_tooltip.innerHTML = element.innerHTML;
    real_tooltip.classList.add("is-active");
}

function hide_tooltip(real_tooltip) {
    'use strict';

    real_tooltip.innerHTML = "";
    real_tooltip.classList.remove("is-active");
}
