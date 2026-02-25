# schema-org-json-ld
library for generating json-ld rich results

Tested & Validates using Google "Rich Results Test": https://search.google.com/test/rich-results/result?id=eeuHnX6wLe1IqxNkqd42xA

PHP library for generating schema.org JSON-LD with broad Google Rich Results coverage (27 rich result types backed by 65 schema classes).

## supported types

| Google Rich Results category | Supported schema types |
| --- | --- |
| Article | `Article`, `NewsArticle`, `BlogPosting` |
| Breadcrumb | `BreadcrumbList` |
| Carousel | `ItemList` |
| Course | `Course` |
| Dataset | `Dataset` |
| Discussion forum | `DiscussionForumPosting` |
| Education Q&A | `Quiz` |
| Employer aggregate rating | `EmployerAggregateRating` |
| Event | `Event` |
| FAQ | `FAQPage` |
| Job posting | `JobPosting` |
| Local business | `LocalBusiness` |
| Movie | `Movie` |
| Organization | `Organization` |
| Product | `Product`, `Offer`, `Brand`, `OfferShippingDetails`, `ShippingDeliveryTime` |
| Profile page | `ProfilePage` |
| Q&A | `QAPage` |
| Recipe | `Recipe` |
| Review snippet | `Review`, `AggregateRating` |
| Software app | `SoftwareApplication`, `MobileApplication`, `WebApplication` |
| Speakable | `SpeakableSpecification` |
| Subscription/paywalled content | `WebPageElement` |
| Vacation rental | `VacationRental` |
| Video | `VideoObject` |


## install
`composer require evabee/schema-org-json-ld`

please note that the alternative vendor name is correct. due to packagist wanting excessive Github permissions i'm locked out of the default vendor namespace.

### example usage

```php
$product = new Product(
	name: "Executive Anvil",
	image: [
		"https://example.com/photos/1x1/photo.jpg",
		"https://example.com/photos/4x3/photo.jpg",
		"https://example.com/photos/16x9/photo.jpg"
	],
	description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
	sku: "0446310786",
	brand: new Brand(
		name: "ACME (tm)",
	),
	mpn: "ACME0444246625",
	weight: new QuantitativeValue(
		value: 55.67,
		unitCode: "LBR"
	),
	offers: [
		new Offer(
			url: "https://example.com/anvil",
			priceCurrency: "USD",
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock,
			shippingDetails: [
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(
						addressCountry: "US",
						addressRegion: [ "CA", "NV", "AZ" ]
					),
					shippingRate: new MonetaryAmount(
						value: 3.49,
						currency: "USD",
					),
					deliveryTime: new ShippingDeliveryTime(
						handlingTime: new QuantitativeValue(
							unitCode: "DAY",
							minValue: 0,
							maxValue: 1
						),
						transitTime: new QuantitativeValue(
							unitCode: "DAY",
							minValue: 1,
							maxValue: 5
						)
					)
				),
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(
						addressCountry: "US",
						addressRegion: [ "HI" ]
					),
					shippingRate: new MonetaryAmount(
						value: 77.49,
						currency: "USD",
					),
					deliveryTime: new ShippingDeliveryTime(
						handlingTime: new QuantitativeValue(
							unitCode: "DAY",
							minValue: 0,
							maxValue: 1
						),
						transitTime: new QuantitativeValue(
							unitCode: "DAY",
							minValue: 4,
							maxValue: 10
						)
					)
				),
				new OfferShippingDetails(
					shippingDestination: new DefinedRegion(
						addressCountry: "US",
						addressRegion: [ "AK" ]
					),
					doesNotShip: true,
				),
			]
		)
	]
);

$json = JsonLdGenerator::SchemaToJson(
	schema: $product
);
```

... this will output:

```json
{
	"@context": "https://schema.org/",
	"@type": "Product",
	"name": "Executive Anvil",
	"image": [
		"https://example.com/photos/1x1/photo.jpg",
		"https://example.com/photos/4x3/photo.jpg",
		"https://example.com/photos/16x9/photo.jpg"
	],
	"description": "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
	"sku": "0446310786",
	"offers": [
		{
			"@type": "Offer",
			"url": "https://example.com/anvil",
			"priceCurrency": "USD",
			"price": 119.99,
			"itemCondition": "https://schema.org/NewCondition",
			"availability": "https://schema.org/InStock",
			"shippingDetails": [
				{
					"@type": "OfferShippingDetails",
					"shippingDestination": {
						"@type": "DefinedRegion",
						"addressCountry": "US",
						"addressRegion": [
							"CA",
							"NV",
							"AZ"
						]
					},
					"shippingRate": {
						"@type": "MonetaryAmount",
						"currency": "USD",
						"value": 3.49
					},
					"deliveryTime": {
						"@type": "ShippingDeliveryTime",
						"handlingTime": {
							"@type": "QuantitativeValue",
							"unitCode": "DAY",
							"minValue": 0,
							"maxValue": 1
						},
						"transitTime": {
							"@type": "QuantitativeValue",
							"unitCode": "DAY",
							"minValue": 1,
							"maxValue": 5
						}
					}
				},
				{
					"@type": "OfferShippingDetails",
					"shippingDestination": {
						"@type": "DefinedRegion",
						"addressCountry": "US",
						"addressRegion": [
							"HI"
						]
					},
					"shippingRate": {
						"@type": "MonetaryAmount",
						"currency": "USD",
						"value": 77.49
					},
					"deliveryTime": {
						"@type": "ShippingDeliveryTime",
						"handlingTime": {
							"@type": "QuantitativeValue",
							"unitCode": "DAY",
							"minValue": 0,
							"maxValue": 1
						},
						"transitTime": {
							"@type": "QuantitativeValue",
							"unitCode": "DAY",
							"minValue": 4,
							"maxValue": 10
						}
					}
				},
				{
					"@type": "OfferShippingDetails",
					"shippingDestination": {
						"@type": "DefinedRegion",
						"addressCountry": "US",
						"addressRegion": [
							"AK"
						]
					},
					"doesNotShip": true
				}
			]
		}
	],
	"brand": {
		"@type": "Brand",
		"name": "ACME (tm)"
	},
	"mpn": "ACME0444246625",
	"weight": {
		"@type": "QuantitativeValue",
		"value": 55.67,
		"unitCode": "LBR"
	}
}

```

### more examples

#### article (simple)

```php
$article = new Article(
	headline: "How to use schema.org JSON-LD in PHP",
	datePublished: "2026-02-25",
);
```

```json
{
	"@type": "Article",
	"headline": "How to use schema.org JSON-LD in PHP"
}
```

#### faq page (simple)

```php
$faq = new FAQPage(
	mainEntity: [
		new Question(
			name: "Do you support FAQ rich results?",
			acceptedAnswer: new Answer(text: "Yes."),
		),
	],
);
```

```json
{
	"@type": "FAQPage",
	"mainEntity": [{ "@type": "Question" }]
}
```

#### event (simple)

```php
$event = new Event(
	name: "PHP SEO Meetup",
	startDate: "2026-03-15T18:00:00+00:00",
	location: new Place(name: "Online"),
);
```

```json
{
	"@type": "Event",
	"name": "PHP SEO Meetup"
}
```

see unit tests directory for more details:
https://github.com/EvaLok/schema-org-json-ld/tree/master/test/unit
