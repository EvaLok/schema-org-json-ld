<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

class Product {

	const A_CONTEXT = 'https://schema.org/';
	const A_SCHEMA_TYPE = 'Product';

	public string $name;

	/**
	 * @var string[]
	 */
	public array $images;

	public string $description;
	public string $sku;


	function __construct(

	) {

	}
}
