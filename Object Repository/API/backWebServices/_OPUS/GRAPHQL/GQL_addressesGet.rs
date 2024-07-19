<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GQL_addressesGet</name>
   <tag></tag>
   <elementGuidId>05909ede-ce69-4dfa-905d-3d3e03ac42cb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;operationName\&quot;: \&quot;GetMyAddresses\&quot;,\n  \&quot;variables\&quot;: { \&quot;sort\&quot;: \&quot;lastName:asc\&quot; },\n  \&quot;query\&quot;: \&quot;query GetMyAddresses($after: String, $first: Int, $sort: String) {\\n  me {\\n    contact {\\n      billingAddresses(after: $after, first: $first, sort: $sort) {\\n        totalCount\\n        items {\\n          ...memberAddressFields\\n        }\\n      }\\n      shippingAddresses(after: $after, first: $first, sort: $sort) {\\n        totalCount\\n        items {\\n          ...memberAddressFields\\n        }\\n      }\\n    }\\n  }\\n}\\n\\nfragment memberAddressFields on MemberAddressType {\\n  id\\n  name\\n  organization\\n  firstName\\n  lastName\\n  line1\\n  line2\\n  city\\n  countryCode\\n  countryName\\n  regionId\\n  regionName\\n  postalCode\\n  isDefault\\n  phone\\n  email\\n  description\\n  addressType\\n}\&quot;\n}\n&quot;,
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
      <defaultValue>'opus'</defaultValue>
      <description></description>
      <id>877818db-82bb-4530-836b-27ece9574709</id>
      <masked>false</masked>
      <name>storeId</name>
   </variables>
   <variables>
      <defaultValue>'bda32fbe-bf09-4944-846b-12b614bdee8f'</defaultValue>
      <description></description>
      <id>144f89c3-ccc1-4f82-9426-7e558d031858</id>
      <masked>false</masked>
      <name>userId</name>
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
