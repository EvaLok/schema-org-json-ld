# schema-org-json-ld
library for generating json-ld rich results

Tested & Validates using Google "Rich Results Test": https://search.google.com/test/rich-results/result?id=eeuHnX6wLe1IqxNkqd42xA

work-in-progress; currently only supports `Product` and `Offer` schema.org `Thing`s, along with some (but not all) of their essential properties. Pull Requests welcome! 


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

see unit tests directory for more details:
https://github.com/EvaLok/schema-org-json-ld/tree/master/test/unit
