<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Certification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Certification';

	public function __construct(
		public string $name,
		public Organization $issuedBy,
		public null|string $certificationIdentification = null,
		public null|Rating $certificationRating = null,
	) {}
}
