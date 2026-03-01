<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class AlignmentObject extends TypedSchema {
	public const A_SCHEMA_TYPE = 'AlignmentObject';

	public function __construct(
		public string $alignmentType,
		public string $targetName,
		public null|string $educationalFramework = null,
		public null|string $targetUrl = null,
	) {}
}
