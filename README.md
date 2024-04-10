# schema-org-json-ld
library for generating json-ld rich results

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
	offers: [
		new Offer(
			url: "https://example.com/anvil",
			priceCurrency: "USD",
			price: 119.99,
			itemCondition: OfferItemCondition::NewCondition,
			availability: ItemAvailability::InStock
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
	"offers": [{
		"@type": "Offer",
		"url": "https://example.com/anvil",
		"priceCurrency": "USD",
		"price": 119.99,
		"itemCondition": "https://schema.org/NewCondition",
		"availability": "https://schema.org/InStock"
	}]
}

```

see unit tests directory for more details:
https://github.com/EvaLok/schema-org-json-ld/tree/master/test/unit
