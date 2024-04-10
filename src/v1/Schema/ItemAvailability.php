<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum ItemAvailability: string {
	case InStock = 'https://schema.org/InStock';
	case OutOfStock = 'https://schema.org/OutOfStock';
	case Discontinued = 'https://schema.org/Discontinued';
}
