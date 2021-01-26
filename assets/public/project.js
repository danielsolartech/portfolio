const imagesSlider = new Splide('#images-slider', {
    type: 'fade',
    heightRatio: 0.5,
    pagination: false,
    arrows: false,
    cover: true,
    autoplay: true,
    rewind: true,
    pauseOnHover: true,
});

const thumbnailsSlider = new Splide('#thumbnails-slider', {
    fixedWidth: 100,
    height: 60,
    gap: 10,
    rewind: true,
    cover: true,
    isNavigation: true,
    pagination: false,
    focus: 'center',
    breakpoints: {
        576: {
            fixedWidth: 60,
            height: 40,
            gap: 5,
        },
    },
});

imagesSlider.sync(thumbnailsSlider.mount()).mount();
