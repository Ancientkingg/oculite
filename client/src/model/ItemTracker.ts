export default class ItemTracker {
    private id: number;
    private name: string;
    private priceData: { dates: Date[]; prices: number[] };
    private currency: string;
    private icon: string;
    private link: string;
    private favorite: boolean;

    constructor(
        id: number,
        name: string,
        priceData: { dates: Date[]; prices: number[] },
        currency: string,
        icon: string,
        link: string,
        favorite: boolean,
    ) {
        this.id = id;
        this.name = name;
        this.priceData = priceData;
        this.currency = currency;
        this.icon = icon;
        this.link = link;
        this.favorite = favorite;
    }

    getId(): number {
        return this.id;
    }

    getName(): string {
        return this.name;
    }

    getPriceData(): { dates: Date[]; prices: number[] } {
        return this.priceData;
    }

    getCurrency(): string {
        return this.currency;
    }

    getIcon(): string {
        return this.icon;
    }

    getLink(): string {
        return this.link;
    }

    isFavorite(): boolean {
        return this.favorite;
    }

    getLatestPrice(): number {
        return this.priceData.prices[this.priceData.prices.length - 1];
    }

    getPreviousPrice(): number {
        return this.priceData.prices[this.priceData.prices.length - 2];
    }

    getPriceChangeAsString(): string {
        const latestPrice = Math.round(this.getLatestPrice() * 100.0) / 100.0;
        const previousPrice =
            Math.round(this.getPreviousPrice() * 100.0) / 100.0;

        const percentageChange = Math.round(
            ((latestPrice - previousPrice) / previousPrice) * 100.0,
        );
        const signSymbol = percentageChange > 0 ? '+' : '';

        return `${signSymbol}${percentageChange}`;
    }
}
