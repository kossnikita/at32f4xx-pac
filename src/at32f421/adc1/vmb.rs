#[doc = "Register `VM%sB` reader"]
pub type R = crate::R<VMB_SPEC>;
#[doc = "Register `VM%sB` writer"]
pub type W = crate::W<VMB_SPEC>;
#[doc = "Field `VMB` reader - Voltage monitoring high boundary"]
pub type VMB_R = crate::FieldReader<u16>;
#[doc = "Field `VMB` writer - Voltage monitoring high boundary"]
pub type VMB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    pub fn vmb(&self) -> VMB_R {
        VMB_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMB")
            .field("vmb", &format_args!("{}", self.vmb().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<VMB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    #[must_use]
    pub fn vmb(&mut self) -> VMB_W<VMB_SPEC> {
        VMB_W::new(self, 0)
    }
}
#[doc = "Voltage monitoring %s boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMB_SPEC;
impl crate::RegisterSpec for VMB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmb::R`](R) reader structure"]
impl crate::Readable for VMB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmb::W`](W) writer structure"]
impl crate::Writable for VMB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VM%sB to value 0x0fff"]
impl crate::Resettable for VMB_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
