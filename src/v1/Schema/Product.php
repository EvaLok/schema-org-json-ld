<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

class Product {
	const A_SCHEMA_TYPE = 'Product';

	function __construct(
		public string $name,
		/** @var string[] $images */
		public array $images,
		public string $description,
		public string $sku,
		/** @var Offer[] $offers */
		public array $offers,
	) {

	}
}
