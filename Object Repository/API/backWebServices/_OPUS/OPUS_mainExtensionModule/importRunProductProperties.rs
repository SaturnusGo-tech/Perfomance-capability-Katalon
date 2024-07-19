<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>importRunProductProperties</name>
   <tag></tag>
   <elementGuidId>e5853b62-278c-4453-9d56-1f595a2750b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;fileUrl\&quot;: \&quot;${fileUrl}\&quot;,\n\t\&quot;delimiter\&quot;: \&quot;${delimiter}\&quot;,\n\t\&quot;supplierId\&quot;: \&quot;${supplierId}\&quot;\n}&quot;,
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
      <webElementGuid>9e8d7034-0b5d-412c-8346-4a45437358ea</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>981e5072-dbdb-4720-9122-c51cfa6e2e14</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/opus/suppliers/productProperties/import/run</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'2a97eda8-da86-4904-a627-d78877594cd8'</defaultValue>
      <description>name of an added supplier</description>
      <id>60c9ce4e-16fe-401c-82fd-e6c5f082c337</id>
      <masked>false</masked>
      <name>supplierId</name>
   </variables>
   <variables>
      <defaultValue>'tmp/propertiesMappings.csv'</defaultValue>
      <description>url of the uploaded csv file</description>
      <id>dccbf832-c788-4931-a3ef-1d6fbd6126f6</id>
      <masked>false</masked>
      <name>fileUrl</name>
   </variables>
   <variables>
      <defaultValue>','</defaultValue>
      <description>selected delimeter</description>
      <id>0b67f7fd-30d0-42f3-82d7-2af5e4764344</id>
      <masked>false</masked>
      <name>delimiter</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

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
