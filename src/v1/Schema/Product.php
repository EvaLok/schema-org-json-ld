<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Product extends TypedSchema {
	const A_SCHEMA_TYPE = 'Product';

	function __construct(
		public string $name,
		/** @var string[] $image */
		public array  $image,
		public string $description,
		public string $sku,
		/** @var Offer[] $offers */
		public array  $offers,
		public null|Brand $brand = null,
		public null|string $mpn = null,
		public null|QuantitativeValue $weight = null,
	) {

	}
}
