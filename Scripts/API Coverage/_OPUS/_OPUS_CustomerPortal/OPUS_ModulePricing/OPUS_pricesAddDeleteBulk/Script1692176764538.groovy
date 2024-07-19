import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable


WebUI.comment('TEST CASE : OPUS PRICES ADD DELETE BULK')


//Use hardcoded products (product#10 & product#100) from OPUS catalog instead of creating them
List productIdList = ['663dd0d4-e695-4e18-97cd-850dde998a90_6ece5305-deba-4321-a6fd-5d9e5e303173','663dd0d4-e695-4e18-97cd-850dde998a90_2269b5bd-2417-42e3-98e9-840423301d77']
GlobalVariable.productId = productIdList


'ADD PRODUCTS TO THE PRICELIST USING PRODUCT IDs FROM THE LIST (one id per loop iteration)'
//loop overwrites GV.productId list, so need to set it back after the loop is done
for(int j;j<productIdList.size();j++) {
	GlobalVariable.productId = productIdList[j]
	WS.callTestCase(findTestCase('API Coverage/ModulePricing/pricelistProductsAdd'),
		null)
}


'GET ADDED PRICES IDS'
priceIdSearch = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Pricing/ProductsPricesGetSearch',[
	('pricelistId') : GlobalVariable.pricelistId
	]))
List priceIdList = WS.getElementPropertyValue(priceIdSearch, 'results.prices.id')


'SET GV.productId BACKTO LIST OF CREATED PRODUCT IDs TO BULK DELETE THEM'
GlobalVariable.productId = productIdList


'DELETE PRICES FROM THE LIST'
for(int n;n<priceIdList.size();n++) {
	GlobalVariable.priceId = priceIdList[n][0]
deletePrices = WS.sendRequestAndVerify(findTestObject('Object Repository/API/backWebServices/VirtoCommerce.Pricing/PriceDeleteByPriceId',[
	('priceId') : GlobalVariable.priceId//productIdList[0] + '&productIds=' + productIdList[1]
	]))
}


'VERIFY PRICES HAVE BEEN DELETED'
for(int n;n<priceIdList.size();n++) {
	GlobalVariable.productId = productIdList[n]
	verifyDeleted = WS.callTestCase(findTestCase('Test Cases/API Coverage/ModulePricing/priceSearchPost'),
		null)
	WS.verifyElementPropertyValue(verifyDeleted,'totalCount', '0')
}


'SET GV.productId BACKTO THE INITIAL VALUE'
GlobalVariable.productId = GlobalVariable.OPUS_productId

