export interface ApiResponse {
	data: Data;
}

export interface Data {
	id: number;
	type: string;
	title: string;
	brandNo: string;
	brand: string;
	genericArticleId: number;
	genericArticle: string;
	sale: number;
	isAutodocPlusSale: boolean;
	qty: number;
	defaultQty: number;
	stepQty: number;
	minQty: number;
	price: Price;
	grandTotal: GrandTotal;
	originalPrice: OriginalPrice;
	isRRP: boolean;
	isInStock: boolean;
	isDisplayVat: boolean;
	categoryIds: string[];
	images: Image[];
	brandImageUrl: string;
	websiteUrl: string;
	hasFreeDelivery: boolean;
	deliveryDate: any;
	isSuitableForAllCars: boolean;
	score: string;
	number: string;
	moneyback: Moneyback;
	moneyBackMessage: any;
	isTop: boolean;
	hasCompressorWarning: boolean;
	isSuitable: boolean;
	possibleQty: any;
	isOen: boolean;
	hasRoundPhoto: boolean;
	info: any;
	bonusInfo: any;
	vatAnnotation: VatAnnotation;
	aboutPageLinks: any[];
	hazard: any;
	premiumDelivery: any;
	priceInfo: PriceInfo;
	brandChoiceMessage: any;
	brandChoiceInfo: any;
	expertChoiceMessage: any;
	youtubeManual: any;
	ean: string;
	bonus: any;
	clubPdfManual: any;
	originalBonusPrice: any;
	deliveryInfo: DeliveryInfo;
	oneyPaymentPlaceholder: any;
	isSurcharge: boolean;
	surchargeMessage: any;
	isDeliverable: boolean;
	warranty: any;
	qualityWarranty: boolean;
	isSafeOrder: boolean;
	description: Description[];
	plusPlanPromo: PlusPlanPromo;
	roundPhoto: RoundPhoto;
	oilCarVolume: any;
	hasMirrorArticle: boolean;
	pdfUrls: any[];
	review: Review;
	marketingDescription: MarketingDescription;
}

export interface Price {
	current: Current;
	default: Default;
}

export interface Current {
	currency: string;
	price: number;
}

export interface Default {
	currency: string;
	price: number;
}

export interface GrandTotal {
	current: Current2;
	default: Default2;
}

export interface Current2 {
	currency: string;
	price: number;
}

export interface Default2 {
	currency: string;
	price: number;
}

export interface OriginalPrice {
	current: Current3;
	default: Default3;
}

export interface Current3 {
	currency: string;
	price: number;
}

export interface Default3 {
	currency: string;
	price: number;
}

export interface Image {
	small: string;
	medium: string;
	big: string;
}

export interface Moneyback {
	current: Current4;
	default: Default4;
}

export interface Current4 {
	currency: string;
	price: number;
}

export interface Default4 {
	currency: string;
	price: number;
}

export interface VatAnnotation {
	text: string;
	links: any[];
}

export interface PriceInfo {
	text: string;
	links: any[];
}

export interface DeliveryInfo {
	button: Button;
	icons: string[];
}

export interface Button {
	text: string;
	pageId: number;
}

export interface Description {
	param: string;
	value: string;
}

export interface PlusPlanPromo {
	iconUrl: string;
	title: string;
	planId: number;
}

export interface RoundPhoto {
	count: number;
	link: string;
	mainPhotoIndex: number;
}

export interface Review {
	stars: number;
	countComments: number;
	reviewsCount: number;
	starsComment: string;
}

export interface MarketingDescription {
	bullets: string[];
	text: string;
}
