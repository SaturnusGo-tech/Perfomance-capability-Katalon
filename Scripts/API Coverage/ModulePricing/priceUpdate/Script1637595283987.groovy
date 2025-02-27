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


WebUI.comment('TEST CASE: price udate')


'UPDATE PRODUCT PRICE'
updatedPrice = '555'
priceUpdate = WS.sendRequestAndVerify(findTestObject('API/backWebServices/VirtoCommerce.Pricing/PricesAddUpdateByProductId', [
	('productId') : GlobalVariable.productId,
	('productName') : GlobalVariable.productName,
	('catalogId') : GlobalVariable.catalogId,
	('pricelistId') : GlobalVariable.pricelistId,
	('priceId') : GlobalVariable.priceId,
	('listPrice') : updatedPrice
	]))


'VERIFY THE PRODUCT & THE PRICE ARE ADDED'
verifyPrice = WS.callTestCase(findTestCase('API Coverage/ModulePricing/priceSearchGet'),
	null
	)
//WS.verifyElementPropertyValue(verifyPrice,'totalCount','1')
WS.verifyElementPropertyValue(verifyPrice,'results[0].prices[0].pricelistId',GlobalVariable.pricelistId)
WS.verifyElementPropertyValue(verifyPrice,'results[0].prices[0].productId',GlobalVariable.productId)
WS.verifyElementPropertyValue(verifyPrice,'results[0].prices[0].list', updatedPrice + '.0000')

