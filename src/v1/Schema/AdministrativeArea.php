<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class AdministrativeArea extends TypedSchema {
	public const A_SCHEMA_TYPE = 'AdministrativeArea';

	public function __construct(
		public string $name,
	) {}
}
