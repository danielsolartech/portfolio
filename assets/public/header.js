const toggleButton = document.getElementById('toggleNav');
const navBox = document.getElementById('nav');

const toggleButtonIcon = bodymovin.loadAnimation({
    container: toggleButton,
    renderer: 'svg',
    loop: false,
    autoplay: false,
    path: '/assets/icons/menuV2.json',
});

const navDuration = 800;

let navDirectionMenu = 1;
toggleButton.addEventListener('click', () => {
    if (navBox.classList.contains('active')) {
        navBox.animate([
            { top: '57px' },
            { top: '-1000px' },
        ], { duration: navDuration, easing: 'ease-in-out' });

        setTimeout(() => navBox.classList.remove('active'), navDuration);
    } else {
        navBox.classList.add('active');
        navBox.animate([
            { top: '-1000px' },
            { top: '57px' }
        ], { duration: navDuration, easing: 'ease-in-out' });
    }

    toggleButtonIcon.setDirection(navDirectionMenu);
    toggleButtonIcon.play();
    navDirectionMenu = -navDirectionMenu;
});
