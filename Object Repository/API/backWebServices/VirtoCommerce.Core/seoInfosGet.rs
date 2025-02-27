<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>seoInfosGet</name>
   <tag></tag>
   <elementGuidId>fbb31daa-e03f-403d-802e-c1b848d75331</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>454a954e-3d1f-4665-be70-af4be8fefe1f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>api_key</name>
      <type>Main</type>
      <value>${GlobalVariable.api_key}</value>
      <webElementGuid>710e2440-4713-426f-951a-05bbc68bb1da</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.urlBack}/api/seoinfos/${slugName}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'qwecategory'</defaultValue>
      <description></description>
      <id>29437f0c-0b85-40d7-b012-824a3e6967c9</id>
      <masked>false</masked>
      <name>slugName</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonOutput
import com.kms.katalon.core.util.KeywordUtil


//No need to parse and return JSON. 
//Because somtimes returned content is in HTML format and this will cause an error.
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
KeywordUtil.logInfo(response.responseBodyContent)
WS.verifyResponseStatusCode(response, 200)
  </verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
