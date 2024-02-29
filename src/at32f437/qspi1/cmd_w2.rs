#[doc = "Register `CMD_W2` reader"]
pub type R = crate::R<CMD_W2_SPEC>;
#[doc = "Register `CMD_W2` writer"]
pub type W = crate::W<CMD_W2_SPEC>;
#[doc = "Field `DCNT` reader - Read write data counter"]
pub type DCNT_R = crate::FieldReader<u32>;
#[doc = "Field `DCNT` writer - Read write data counter"]
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read write data counter"]
    #[inline(always)]
    pub fn dcnt(&self) -> DCNT_R {
        DCNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_W2")
            .field("dcnt", &format_args!("{}", self.dcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMD_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read write data counter"]
    #[inline(always)]
    #[must_use]
    pub fn dcnt(&mut self) -> DCNT_W<CMD_W2_SPEC> {
        DCNT_W::new(self, 0)
    }
}
#[doc = "Command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_W2_SPEC;
impl crate::RegisterSpec for CMD_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w2::R`](R) reader structure"]
impl crate::Readable for CMD_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_w2::W`](W) writer structure"]
impl crate::Writable for CMD_W2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_W2 to value 0x0100_0003"]
impl crate::Resettable for CMD_W2_SPEC {
    const RESET_VALUE: u32 = 0x0100_0003;
}
