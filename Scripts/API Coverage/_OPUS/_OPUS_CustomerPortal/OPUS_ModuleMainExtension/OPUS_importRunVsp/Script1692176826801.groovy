import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable



WebUI.comment('TEST CASE: run products import and check if its finished')


'GET INITIAL IMPORT QUERY NUMBERR'//to verify the examined operation is the expected one
WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Platform/PushNotificationMarkAsRead'))
getImportQueryHistory= WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importHistorySearch',[
	('userId') : GlobalVariable.supplierId
	]))
eventsInitialCount = WS.getElementPropertyValue(getImportQueryHistory,'totalCount')
println eventsInitialCount 


'RUN IMPORT'
importRun = WS.sendRequest(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importRun',[
	('supplierId') : GlobalVariable.supplierId
	]))


'VERIFY IMPORT OPERATION IS LOGGED AS AN IMPORT QUERY'
eventsActualCount = eventsInitialCount
while (eventsActualCount == eventsInitialCount) {
	getRecentImportHistory = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importHistorySearch',[
	('userId') : GlobalVariable.supplierId
	]))
	eventsActualCount = WS.getElementPropertyValue(getRecentImportHistory,'totalCount')
	//importFinishingStatus = WS.getElementPropertyValue(getRecentImportHistory,'notifyEvents[0].finished')
	//println eventsInitialCount 	
	println eventsInitialCount 
	println eventsActualCount 
	}


'VERIFY IMPORT OPERATION HAS BEEN FINSHED AND COLLECT JOBID TO PUBLISH RESULTS'
def importFinishingStatus
def jobId
int i
while (importFinishingStatus == null) {
	i++
	getRecentImportHistory = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importHistorySearch',[
	('userId') : GlobalVariable.supplierId
	]))
	importFinishingStatus = WS.getElementPropertyValue(getRecentImportHistory,'results[0].finished')
	println importFinishingStatus
	errorsCount = WS.getElementPropertyValue(getRecentImportHistory,'results[0].errors')
	assert 	( errorsCount == [] ) || ( errorsCount == null )
	println errorsCount
	importType = WS.getElementPropertyValue(getRecentImportHistory,'results[0].typeName') 
	assert importType == 'ImportRunHistory'
	println importType
	jobId = WS.getElementPropertyValue(getRecentImportHistory, 'results[0].jobId')
	if (i == 150) {
		throw new com.kms.katalon.core.exception.StepFailedException('ERROR: import verification timeout')
		}
	}
	
	
'PUBLISH IMPORT RESULTS'//can only be verified via 200OK or later catalog related tests
importPublish = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/_OPUS/OPUS_mainExtensionModule/importSwapResults',[
	('supplierId') : GlobalVariable.supplierId
	]))

