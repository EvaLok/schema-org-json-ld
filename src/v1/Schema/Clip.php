<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Clip extends TypedSchema {
	public const A_SCHEMA_TYPE = 'Clip';

	public function __construct(
		public string $name,
		public int $startOffset,
		public string $url,
		public null|int $endOffset = null,
	) {}
}
