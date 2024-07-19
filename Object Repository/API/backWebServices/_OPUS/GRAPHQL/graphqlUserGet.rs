<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>graphqlUserGet</name>
   <tag></tag>
   <elementGuidId>f009e0af-bd8f-4c58-bc13-db0e5054a9ca</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation addItem($command: InputAddItemType!) { addItem(command: $command) { id customerId items { id name productId quantity } validationErrors {errorMessage } } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;command\\\&quot;: { \\\&quot;userId\\\&quot;: \\\&quot;30f99c97-83f5-4154-9edb-b5d79d3d5397\\\&quot;, \\\&quot;storeId\\\&quot;: \\\&quot;katalonStore\\\&quot;, \\\&quot;productId\\\&quot;: \\\&quot;2dcd49147dc04892892af26bb91e5530\\\&quot;, \\\&quot;quantity\\\&quot;: 1 }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation addItem($command: InputAddItemType!) {\n  addItem(command: $command) {\n\n    id\n    customerId\n    items\n    {\n      id\n      name\n      productId\n      quantity\n    }\n    validationErrors\n    {errorMessage\n    }\n  }\n  }&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;command\&quot;: {\n    \&quot;userId\&quot;: \&quot;30f99c97-83f5-4154-9edb-b5d79d3d5397\&quot;,\n  \&quot;storeId\&quot;:  \&quot;katalonStore\&quot;,\n    \&quot;productId\&quot;: \&quot;2dcd49147dc04892892af26bb91e5530\&quot;,\n    \&quot;quantity\&quot;: 1\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
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
