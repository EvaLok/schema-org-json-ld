<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class PostalAddress extends TypedSchema {
	public const A_SCHEMA_TYPE = 'PostalAddress';

	public function __construct(
		public null|string $streetAddress = null,
		public null|string $addressLocality = null,
		public null|string $addressRegion = null,
		public null|string $postalCode = null,
		public null|string $addressCountry = null,
		public null|string $postOfficeBoxNumber = null,
	) {}
}
