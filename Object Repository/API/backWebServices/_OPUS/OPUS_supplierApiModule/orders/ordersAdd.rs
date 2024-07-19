<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ordersAdd</name>
   <tag></tag>
   <elementGuidId>ac9bc105-8f5d-47d4-aee6-cb224726017a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;number\&quot;: \&quot;string\&quot;,\n  \&quot;supplierAgencyId\&quot;: \&quot;string\&quot;,\n  \&quot;createdDate\&quot;: \&quot;2023-02-05T19:43:51.759Z\&quot;,\n  \&quot;lineItems\&quot;: [\n    {\n      \&quot;sku\&quot;: \&quot;string\&quot;,\n      \&quot;quantity\&quot;: 0\n    }\n  ],\n  \&quot;comment\&quot;: \&quot;string\&quot;,\n  \&quot;shippingAddress\&quot;: {\n    \&quot;countryCode\&quot;: \&quot;string\&quot;,\n    \&quot;stateCode\&quot;: \&quot;string\&quot;,\n    \&quot;city\&quot;: \&quot;string\&quot;,\n    \&quot;postalCode\&quot;: \&quot;string\&quot;,\n    \&quot;line1\&quot;: \&quot;string\&quot;,\n    \&quot;line2\&quot;: \&quot;string\&quot;,\n    \&quot;firstName\&quot;: \&quot;string\&quot;,\n    \&quot;lastName\&quot;: \&quot;string\&quot;,\n    \&quot;phone\&quot;: \&quot;string\&quot;,\n    \&quot;email\&quot;: \&quot;string\&quot;\n  }\n}&quot;,
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
      <webElementGuid>814b9db6-6b28-4764-840d-48655d093e5d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>c4986d3b-6505-437c-86ad-2f00479e7093</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}api/v1.0/order?supplierId=${supplierId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'qwe123'</defaultValue>
      <description></description>
      <id>662fa683-822e-4f72-b637-1a07d24cdb3b</id>
      <masked>false</masked>
      <name>supplierId</name>
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
//Status verification is turned off as it returns empty response
//WS.verifyResponseStatusCode(response, 204)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
