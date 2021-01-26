const header = document.getElementsByTagName('header')[0];
const toggleButton = document.getElementById('toggleNav');
const navBox = document.getElementById('nav');

let lastScroll = window.scrollY;
const headerDuration = 500;

window.onscroll = () => {
    const currentScroll = window.scrollY;

    if (currentScroll === 0) {
        if (header.classList.contains('fixed')) {
            header.classList.remove('fixed');
        }
    } else {
        if (!header.classList.contains('fixed')) {
            header.classList.add('fixed');
        }

        if (currentScroll > lastScroll) {
            if (!header.classList.contains('no-show')) {
                header.animate([
                    { top: `0px` },
                    { top: `-${header.clientHeight}px` },
                ], { duration: headerDuration, easing: 'ease-in-out' });
                setTimeout(() => header.classList.add('no-show'), headerDuration);
            }
        } else {
            if (header.classList.contains('no-show')) {
                header.classList.remove('no-show');
                header.animate([
                    { top: `-${header.clientHeight}px` },
                    { top: '0px' },
                ], { duration: headerDuration, easing: 'ease-in-out' });
            }
        }
    }

    lastScroll = currentScroll;
};

const toggleButtonIcon = bodymovin.loadAnimation({
    container: toggleButton,
    renderer: 'svg',
    loop: false,
    autoplay: false,
    path: '/assets/icons/menuV2.json',
});

const duration = 1400;
const easing = 'ease-in-out';

let directionMenu = 1;
toggleButton.addEventListener('click', () => {
    if (navBox.classList.contains('active')) {
        navBox.animate([
            { top: '57px' },
            { top: '-1000px' },
        ], { duration, easing });

        setTimeout(() => navBox.classList.remove('active'), duration);
    } else {
        navBox.classList.add('active');
        navBox.animate([
            { top: '-1000px' },
            { top: '57px' }
        ], { duration, easing });
    }

    toggleButtonIcon.setDirection(directionMenu);
    toggleButtonIcon.play();
    directionMenu = -directionMenu;
});