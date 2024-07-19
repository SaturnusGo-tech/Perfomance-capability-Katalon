<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_cartAddShipmentAddress</name>
   <tag></tag>
   <elementGuidId>03c5344d-4a26-4116-99ec-1cc0bcb090c0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;operationName\&quot;: \&quot;AddOrUpdateCartShipment\&quot;,\n\t\&quot;variables\&quot;: {\n\t\t\&quot;command\&quot;: {\n\t\t\t\&quot;storeId\&quot;: \&quot;${storeId}\&quot;,\n\t\t\t\&quot;userId\&quot;: \&quot;${userId}\&quot;,\n\t\t\t\&quot;cultureName\&quot;: \&quot;en-US\&quot;,\n\t\t\t\&quot;currencyCode\&quot;: \&quot;USD\&quot;,\n\t\t\t\&quot;cartId\&quot;: \&quot;${cartId}\&quot;,\n\t\t\t\&quot;shipment\&quot;: {\n\t\t\t\t\&quot;deliveryAddress\&quot;: ${address},\n\t\t\t\t\&quot;vendorId\&quot;: \&quot;${vendorId}\&quot;\n\t\t\t}\n\t\t}\n\t},\n\t\&quot;query\&quot;: \&quot;mutation AddOrUpdateCartShipment($command: InputAddOrUpdateCartShipmentType!) {\\n  addOrUpdateCartShipment(command: $command) {\\n    ...fullCartFields\\n  }\\n}\\n\\nfragment fullCartFields on OpusCartType {\\n  id\\n  itemsQuantity\\n  purchaseOrderNumber\\n  comment\\n  availableGifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n  }\\n  availableShippingMethods {\\n    id\\n    code\\n    name\\n    logoUrl\\n    optionName\\n    optionDescription\\n    price {\\n      ...moneyFields\\n    }\\n    vendorId\\n  }\\n  availablePaymentMethods {\\n    code\\n    name\\n    description\\n    logoUrl\\n    paymentMethodGroupType\\n  }\\n  items {\\n    ...lineItemFields\\n  }\\n  gifts {\\n    id\\n    imageUrl\\n    name\\n    lineItemId\\n    quantity\\n  }\\n  coupons {\\n    code\\n    isAppliedSuccessfully\\n  }\\n  discounts {\\n    description\\n    amount\\n    coupon\\n  }\\n  shipments {\\n    id\\n    shipmentMethodCode\\n    shipmentMethodOption\\n    deliveryAddress {\\n      ...cartAddressFields\\n    }\\n    price {\\n      ...moneyFields\\n    }\\n    vendor {\\n      id\\n    }\\n    comment\\n  }\\n  payments {\\n    id\\n    paymentGatewayCode\\n    billingAddress {\\n      ...cartAddressFields\\n    }\\n    vendor {\\n      id\\n    }\\n    amount {\\n      amount\\n    }\\n    comment\\n    generalLedgerNumber\\n    purchaseOrderNumber\\n    requisitionNumber\\n    dynamicProperties {\\n      name\\n    }\\n  }\\n  currency {\\n    ...currencyFields\\n  }\\n  total {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  subTotal {\\n    ...moneyFields\\n  }\\n  shippingTotal {\\n    ...moneyFields\\n  }\\n  taxTotal {\\n    ...moneyFields\\n  }\\n  vendors {\\n    vendor {\\n      id\\n      supplier {\\n        paymentMethods\\n        isOrderAllowed\\n        outerId\\n      }\\n    }\\n    subTotal {\\n      amount\\n    }\\n    shippingTotal {\\n      amount\\n    }\\n    taxTotal {\\n      amount\\n    }\\n  }\\n  warnings {\\n    ...validationErrorFields\\n  }\\n  validationErrors(ruleSet: \\\&quot;*\\\&quot;) {\\n    ...validationErrorFields\\n  }\\n  approvalFlowType\\n}\\n\\nfragment cartAddressFields on CartAddressType {\\n  id\\n  name\\n  organization\\n  firstName\\n  lastName\\n  line1\\n  line2\\n  city\\n  countryCode\\n  countryName\\n  regionId\\n  regionName\\n  postalCode\\n  phone\\n  email\\n  addressType\\n  description\\n}\\n\\nfragment lineItemFields on LineItemType {\\n  id\\n  name\\n  sku\\n  quantity\\n  inStockQuantity\\n  imageUrl\\n  selectedForCheckout\\n  productId\\n  productType\\n  product {\\n    id\\n    slug\\n    minQuantity\\n    maxQuantity\\n    masterVariation {\\n      id\\n      slug\\n    }\\n    properties {\\n      ...propertyFields\\n    }\\n    keyProperties {\\n      ...propertyFields\\n    }\\n    availabilityData {\\n      isEstimated\\n    }\\n  }\\n  vendor {\\n    id\\n    name\\n    rating {\\n      value\\n      reviewCount\\n    }\\n    supplier {\\n      paymentMethods\\n      contractNumber\\n      isOrderAllowed\\n    }\\n  }\\n  extendedPrice {\\n    ...moneyFields\\n  }\\n  placedPrice {\\n    ...moneyFields\\n  }\\n  listPrice {\\n    ...moneyFields\\n  }\\n  salePrice {\\n    ...moneyFields\\n  }\\n  discountTotal {\\n    ...moneyFields\\n  }\\n  validationErrors {\\n    errorCode\\n    errorMessage\\n    errorParameters {\\n      key\\n      value\\n    }\\n  }\\n}\\n\\nfragment propertyFields on Property {\\n  name\\n  value\\n  type\\n  hidden\\n  valueType\\n  label\\n}\\n\\nfragment moneyFields on MoneyType {\\n  amount\\n  formattedAmount\\n  formattedAmountWithoutCurrency\\n  currency {\\n    ...currencyFields\\n  }\\n}\\n\\nfragment currencyFields on CurrencyType {\\n  code\\n  symbol\\n}\\n\\nfragment validationErrorFields on ValidationErrorType {\\n  errorCode\\n  errorMessage\\n  errorParameters {\\n    key\\n    value\\n  }\\n  objectId\\n  objectType\\n}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0141233-e5b2-4c87-ad29-33a320387433</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
      <webElementGuid>776c4bc5-779d-4f0d-beb8-c96a5e92805e</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>877818db-82bb-4530-836b-27ece9574709</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>35b1deab-f13f-4e3d-915d-4b1636f1f8d2</id>
      <masked>false</masked>
      <name>cartId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>261da043-9cca-4e73-a881-f93a2eb5b955</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b84d8f5a-c7c0-4bbc-9197-24735d73fbb7</id>
      <masked>false</masked>
      <name>vendorId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d1f6f3f3-99c2-462c-8214-c9266f27a285</id>
      <masked>false</masked>
      <name>shipmentId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def prettyJson = JsonOutput.prettyPrint(response.getResponseBodyContent())
KeywordUtil.logInfo(prettyJson)
WS.verifyResponseStatusCode(response, 200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
