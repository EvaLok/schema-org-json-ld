<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class BedDetails extends TypedSchema {
	public const A_SCHEMA_TYPE = 'BedDetails';

	public function __construct(
		public int $numberOfBeds,
		public null|string $typeOfBed = null,
	) {}
}
