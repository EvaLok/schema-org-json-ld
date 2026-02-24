<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class BreadcrumbList extends TypedSchema {
	public const A_SCHEMA_TYPE = 'BreadcrumbList';

	/**
	 * @param ListItem[] $itemListElement
	 */
	public function __construct(
		public array $itemListElement,
	) {}
}
