<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class VirtualLocation extends TypedSchema {
	public const A_SCHEMA_TYPE = 'VirtualLocation';

	public function __construct(
		public string $url,
		public null|string $name = null,
	) {}
}
