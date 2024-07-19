package objects

import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import org.junit.Assert
import org.openqa.selenium.WebElement
import java.util.Random
import java.util.List

import org.openqa.selenium.WebDriver
import org.openqa.selenium.JavascriptExecutor
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.support.ui.WebDriverWait

import java.net.HttpURLConnection
import java.net.URL

/**
 * The ShippingPage class encapsulates actions and behaviors specific to the shipping page.
 */
class ShippingPage {

	static final String SHIPPING_ADDRESS = 'SHIPPING_ADDRESS'
	static final String DELIVERY_ADDRESS = 'DELIVERY_ADDRESS'
	static final String OPTION = 'OPTION'
	static final String PROCCED_TO_BILLING = 'PROCCED_TO_BILLING'
	static final String ACCEPT_DATA = 'ACCEPT_DATA'

	/**
	 * Navigates through shipping options on the shipping page.
	 */
	static void navigateThroughShippingOptions() {
		// Click on the Shipping Address
		Assert.assertTrue("Shipping Address button is not clickable.",
				WebUI.waitForElementClickable(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + SHIPPING_ADDRESS), 10))
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + SHIPPING_ADDRESS))

		// Click on the Select button
		openUpSeelectBtn()

		// Delay for synchronization
		WebUI.delay(5)

		// Click on Accept Data
		Assert.assertTrue("Accept Data button is not clickable.",
				WebUI.waitForElementClickable(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + ACCEPT_DATA), 10))
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + ACCEPT_DATA))

		// Click on Delivery Address
		Assert.assertTrue("Delivery Address button is not clickable.",
				WebUI.waitForElementClickable(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + DELIVERY_ADDRESS), 10))
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + DELIVERY_ADDRESS))

		// Click on Option
		Assert.assertTrue("Option button is not clickable.",
				WebUI.waitForElementClickable(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + OPTION), 10))
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + OPTION))

		// Delay for synchronization
		WebUI.delay(10)

		// Click on Proceed to Billing
		Assert.assertTrue("Proceed to Billing button is not clickable.",
				WebUI.waitForElementClickable(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + PROCCED_TO_BILLING), 10))
		WebUI.click(ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + PROCCED_TO_BILLING))
	}


	static final String SELECT_ADDRESS = 'SELECT_ADDRESS'

	/**
	 * Clicks the first 'Select' button found on the shipping page.
	 * Throws an exception if no 'Select' buttons are found.
	 */
	static void openUpSeelectBtn() {
		TestObject selectButtonTestObject = ObjectRepository.findTestObject('Object Repository/FRONT_UI/objects/ShippingPage/' + SELECT_ADDRESS)

		Assert.assertTrue("Select Address button is not present.",
				WebUI.waitForElementPresent(selectButtonTestObject, 10))

		List<WebElement> selectButtons = WebUiCommonHelper.findWebElements(selectButtonTestObject, 10)

		WebUI.comment("Found " + selectButtons.size() + " 'Select' buttons.")

		if (!selectButtons.isEmpty()) {
			WebElement firstButton = selectButtons.get(0)
			WebUI.comment("Clicking on the first 'Select' button")
			firstButton.click()
		} else {
			throw new WebElementNotFoundException("No 'Select' buttons found.")
		}
	}
	/**
	 * Checks the page load performance by verifying the page URL and HTTP status.
	 * @param pageUrl The URL of the page to be checked.
	 * @param timeout The timeout for page load in milliseconds.
	 */
	static void checkPageLoadPerformance(String pageUrl, int timeout) {
		// Check the HTTP status of the page
		HttpURLConnection connection = (HttpURLConnection) (new URL(pageUrl).openConnection())
		connection.setRequestMethod("GET")
		connection.connect()
		int responseCode = connection.getResponseCode()
		assert responseCode == 200 : "Expected HTTP status 200, but received " + responseCode

		// Measure the page load time
		WebDriver driver = DriverFactory.getWebDriver()
		long startTime = System.currentTimeMillis()
		driver.get(pageUrl)

		// Convert timeout from milliseconds to seconds for WebDriverWait
		int timeoutInSeconds = timeout / 1000
		new WebDriverWait(driver, timeoutInSeconds).until({ WebDriver d ->
			return ((JavascriptExecutor) d).executeScript("return document.readyState").equals("complete")
		} as com.google.common.base.Function<WebDriver, Boolean>)

		long endTime = System.currentTimeMillis()

		// Page load time in milliseconds
		long loadTime = endTime - startTime
		println("Page load time: " + loadTime + " ms")

		// Check if the page load time exceeds the specified timeout
		assert loadTime <= timeout : "Page load time exceeds " + timeout + " ms"
	}
}

