<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum ReturnFeesEnumeration: string {
	case FreeReturn = 'https://schema.org/FreeReturn';
	case ReturnFeesCustomerResponsibility = 'https://schema.org/ReturnFeesCustomerResponsibility';
	case ReturnShippingFees = 'https://schema.org/ReturnShippingFees';
}
