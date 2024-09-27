interface Item {
	id: number;
	name: string;
	currency: string;
	icon: string;
	link: string;
	price_data: number;
}

function simulateCurrentPrice(
	targetPrice: number, // The price around which fluctuations will hover
	time: number, // A time parameter influencing fluctuation
    volatility: number // A volatility parameter influencing fluctuation
): number {
	const drift = 0.0;
	const dt = time;

	const randomShock = Math.random();
	const standardNormal =
		Math.sqrt(-2 * Math.log(randomShock)) *
		Math.cos(2 * Math.PI * randomShock);

	const priceChangeFactor = Math.exp(
		(drift - 0.5 * volatility ** 2) * dt +
			volatility * Math.sqrt(dt) * standardNormal
	);

	const simulatedPrice = targetPrice * priceChangeFactor;

	return Math.max(simulatedPrice, Math.random()*2);
}

export default (time: number): Item[] => {
    time = Math.round(time / 100) * 100 % 1000;

    return [
        {
            id: 1,
            name: 'Toyota Avensis Remmen',
            currency: '€',
            icon: 'https://www.remdiscounter.nl/media/products/remschijven-toyota-avensis-stationwagen-t27-18-zrt271-30881_17608.jpg',
            link: 'https://www.google.com/search?q=toyota+avensis+remmen',
            price_data: simulateCurrentPrice(80, time, 0.001*2),
        },
        {
            id: 2,
            name: 'Lays Chips Naturel',
            currency: '€',
            icon: 'https://weststrate.nl/media/media_cached/1x1_white/9/f/c/e/lay-s-chips-naturel-zakje-40gr-37152--0.jpg',
            link: 'https://www.google.com/search?q=lays+chips+naturel',
            price_data: simulateCurrentPrice(4.99, time, 0.01),
        },
        {
            id: 3,
            name: 'LEGO Ford Mustang Shelby® GT500®',
            currency: '€',
            icon: 'https://www.lego.com/cdn/cs/set/assets/blt4bd79c45c7dcd055/42138.png?fit=bounds&format=png&width=1500&height=1500&dpr=1',
            link: 'https://www.lego.com/nl-nl/product/ford-mustang-shelby-gt500-42138',
            price_data: simulateCurrentPrice(49.99, time, 0.015),
        },
        {
            id: 4,
            name: 'Large Inflatable Unicorn',
            currency: '€',
            icon: 'https://m.media-amazon.com/images/I/61bvzlYuvGL.jpg',
            link: 'https://www.amazon.nl/-/en/Bramble-Large-Inflatable-Unicorn-200x100x90cm/dp/B079FXZJ3V',
            price_data: simulateCurrentPrice(30, time, 0.01*3),
        },
        {
            id: 5,
            name: 'Pet Rock',
            currency: '€',
            icon: 'https://cdn.accentuate.io/6767454552113/1674587377851/ASW23009-gray-tight-pet-rock.jpg?v=1674587377851',
            link: 'https://en.wikipedia.org/wiki/Pet_Rock',
            price_data: simulateCurrentPrice(12, time, 0.01*4),
        },
        {
            id: 6,
            name: 'Banana Phone',
            currency: '€',
            icon: 'https://www.drunkmall.com/wp-content/uploads/2016/04/Banana-Phone-Case.jpg',
            link: 'https://www.youtube.com/watch?v=j5C6X9vOEkU',
            price_data: simulateCurrentPrice(64, time, 0.01*2),
        },
        {
            id: 7,
            name: 'Bacon Scented Candle',
            currency: '€',
            icon: 'https://dyozopqfp8ikx.cloudfront.net/images/baconaddicts/71/big/bacon-candle.jpg?1500368720',
            link: 'https://www.wikihow.com/Make-Bacon-Candles',
            price_data: simulateCurrentPrice(4, time, 0.01*4),
        },
        {
            id: 8,
            name: 'Pizza Pouch Necklace',
            currency: '€',
            icon: 'https://m.media-amazon.com/images/I/61nAyG4EtKL.jpg',
            link: 'https://hips.hearstapps.com/cosmopolitan-uk/15/40/1443788346-enhanced-5663-1443670673-1.png',
            price_data: simulateCurrentPrice(8, time, 0.01),
        },
        {
            id: 9,
            name: 'Fish Slippers',
            currency: '€',
            icon: 'https://i.ebayimg.com/images/g/xaoAAOSwgjFfmRDb/s-l1000.jpg',
            link: 'https://i.pinimg.com/736x/8e/3b/6c/8e3b6c0f4c45b701e032d671453c6ab6.jpg',
            price_data: simulateCurrentPrice(20, time, 0.01),
        },
        {
            id: 10,
            name: 'A Gold Bar',
            currency: '€',
            icon: 'https://ae-pic-a1.aliexpress-media.com/kf/H127079bc0b3f42958236c6719a7ec172e.jpg_640x640Q90.jpg_.webp',
            link: 'https://www.thesilvermountain.nl/en/buy-gold/gold-bars',
            price_data: simulateCurrentPrice(850, time, 0.01),
        },
    ];
}



