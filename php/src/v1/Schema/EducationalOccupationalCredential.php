<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class EducationalOccupationalCredential extends TypedSchema {
	public const A_SCHEMA_TYPE = 'EducationalOccupationalCredential';

	public function __construct(
		public string $credentialCategory,
	) {}
}
