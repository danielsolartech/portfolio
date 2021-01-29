const toggleContent = document.getElementById('toggleContentIcon');
const blogPostContent = document.getElementById('blogPostContent');

const toggleContentIcon = bodymovin.loadAnimation({
    container: toggleContent,
    renderer: 'svg',
    loop: false,
    autoplay: false,
    path: '/assets/icons/plusToX.json',
});

const blogPostDuration = 1400;
const blogPostClass = 'blog-post__titles__content--no-show';

let contentDirectionMenu = 1;
toggleContent.addEventListener('click', () => {
    if (blogPostContent.classList.contains(blogPostClass)) {
        blogPostContent.classList.remove(blogPostClass);
        blogPostContent.animate([
            { top: '-1000px' },
            { top: '0px' },
        ], { duration: blogPostDuration, easing: 'ease-in-out' });
    } else {
        blogPostContent.animate([
            { top: '0px' },
            { top: '-1000px' },
        ], { duration: blogPostDuration, easing: 'ease-in-out' });
        setTimeout(() => blogPostContent.classList.add(blogPostClass), blogPostDuration);
    }

    toggleContentIcon.setDirection(contentDirectionMenu);
    toggleContentIcon.play();
    contentDirectionMenu = -contentDirectionMenu;
});
