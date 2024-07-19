import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE: import products via sftp process')


'GET ADMIN TOKEN TO AUTHORIZE FURTHER REQUEST'
authorize = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePlatform/Authorization/AuthBackToken'),
	null)


'IMPORT SUPPLIER PRODUCTS AND GET JOB ID'
importRun = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRunSFTP',[
	('supplierId') : GlobalVariable.supplierId,
	('uploadFileUrl') : GlobalVariable.OPUS_productsCsvUrl
	]))
jobId = WS.getElementPropertyValue(importRun,'jobId')
println jobId


'VERIFY JOB HAS BEEN STARTED'
def importRunProgressGetResponseParsed
while ( importRunProgressGetResponseParsed == null) {
	importRunProgressGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRunProgressGet',[
	('jobId') : jobId
	]))
	importRunProgressGetResponse = importRunProgressGet.getResponseBodyContent()
	importRunProgressGetResponseParsed = new JsonSlurper().parseText(importRunProgressGetResponse)
	println importRunProgressGetResponseParsed
}


'VERIFY JOB IS FINISHED' 
def importEndDate
while ( importEndDate == null) {
	importRunProgressGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRunProgressGet',[
	('jobId') : jobId
	]))
	importEndDate = WS.getElementPropertyValue(importRunProgressGet,'finished')
	println importEndDate
	
}


'VERIFY IMPORT HAS BEEN SUCCESSFULL'
importResultsGet = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRunProgressGet',[
	('jobId') : jobId
	]))
errors = WS.getElementPropertyValue(importResultsGet,'errors')
totalCount = WS.getElementPropertyValue(importResultsGet,'totalCount')
processedCount = WS.getElementPropertyValue(importResultsGet,'processedCount')



'PUBLISH IMPORT RESULTS'//can only be verified via 200OK or later catalog related tests
importPublish = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importSwapResults',[
	('jobId') : jobId
	]))


'VERIFY IMPORT RESULTS'
println errors
println totalCount
println processedCount
assert ( errors == [] ) || ( errors == null )
assert (totalCount == processedCount == 100)

