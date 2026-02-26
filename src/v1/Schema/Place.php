<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Place extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Place';

	public function __construct(
		public string $name,
		public null|PostalAddress $address = null,
		public null|GeoCoordinates|GeoShape $geo = null,
	) {}
}
