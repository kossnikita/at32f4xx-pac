#[doc = "Register `VM%sB` reader"]
pub type R = crate::R<VMB_SPEC>;
#[doc = "Register `VM%sB` writer"]
pub type W = crate::W<VMB_SPEC>;
#[doc = "Field `VMB` reader - Voltage monitoring high boundary"]
pub type VMB_R = crate::FieldReader<u16>;
#[doc = "Field `VMB` writer - Voltage monitoring high boundary"]
pub type VMB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Voltage monitoring high boundary"]
    #[inline(always)]
    pub fn vmb(&self) -> VMB_R {
        VMB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMB").field("vmb", &self.vmb()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Voltage monitoring high boundary"]
    #[inline(always)]
    pub fn vmb(&mut self) -> VMB_W<'_, VMB_SPEC> {
        VMB_W::new(self, 0)
    }
}
#[doc = "Voltage monitoring %s boundary register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMB_SPEC;
impl crate::RegisterSpec for VMB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmb::R`](R) reader structure"]
impl crate::Readable for VMB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmb::W`](W) writer structure"]
impl crate::Writable for VMB_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VM%sB to value 0xffff"]
impl crate::Resettable for VMB_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
